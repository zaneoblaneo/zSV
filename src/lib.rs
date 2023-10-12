pub mod z_sv {

    use std::ffi::CString;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ZSV {
        pub data: String,
    }

    impl ZSV {
        /// Finds the first instance of `f` in `self.data`, and returns either
        /// `Some(index)`, or `None` if the character doesn't exist within
        /// `self.data`.
        pub fn find_char(&self, f: char) -> Option<usize> {
            let mut indx: usize = 0usize;
            for c in self.data.chars() {
                if c == f {
                    return Some(indx);
                } else {
                    indx += 1;
                }
            }
            None
        }
        
        /// Finds the first instance of `f` in `self.data`, and returns either
        /// `Some(index)`, or `None` if the string isn't contained within
        /// `self.data`. if `f.data.len()` is 0, we will always return 
        /// `Some(0)`
        pub fn find_string(&self, f: &ZSV) -> Option<usize> {
            if f.data.len() > self.data.len() {
                return None;
            }
            if f.data.len() == 0 {
                return Some(0usize);
            }
            let data_char_array_cache: Vec<char> = self.data.chars().collect();
            let f_char_array_cache: Vec<char> = f.data.chars().collect();
            'outter: for a in 0..self.data.len() {
                for b in 0..f.data.len() {
                    if data_char_array_cache[a+b] != 
                        f_char_array_cache[b] {
                        continue 'outter;
                    }
                }
                return Some(a);
            }
            None
        }
        
        /// Finds the last instance of `f` in `self.data`, and returns either
        /// `Some(index)`, or `None` if the character doesn't exist within
        /// `self.data`.
        pub fn rfind_char(&self, f: char) -> Option<usize> {
            let mut indx: usize = self.data.len();
            for c in self.data.chars().rev() {
                if c == f {
                    return Some(indx);
                } else {
                    indx -= 1;
                }

            }
            None
        }

        /// Finds the last instance of `f` in `self.data`, and returns either
        /// `Some(index)`, or `None` if the string isn't contained within
        /// `self.data`. if `f.data.len()` is 0, we will always return 
        /// `Some(self.data.len()-1)`. if `self.data.len()` is 0, we will 
        /// return None.
        pub fn rfind_string(&self, f: &ZSV) -> Option<usize> {
            if f.data.len() > self.data.len() {
                return None;
            }
            if self.data.len() == 0 {
                return None;
            }
            if f.data.len() == 0 {
                return Some(self.data.len()-1);
            }
            let data_char_array_cache: Vec<char> = self.data.chars().collect();
            let f_char_array_cache: Vec<char> = f.data.chars().collect();
            'outter: for a in (0..=(self.data.len() - f.data.len())).rev() {
                for b in 0..f.data.len() {
                    if data_char_array_cache[a+b] != 
                        f_char_array_cache[b] {
                        continue 'outter;
                    }
                }
                return Some(a);
            }
            None
        }

        /// Splits a ZSV into a tuple of `(a: Option<ZSV>, b: Option<ZSV>)` 
        /// where `Some(a)` is all data left of the first instance of the 
        /// seperator character `s`, and `Some(b)` is all data to the right
        /// of and including the seperator character `s`.
        pub fn split_char(&self, s: char) -> (Option<ZSV>, Option<ZSV>) {
            if let Some(seperator) = self.find_char(s) {
                if seperator >= self.data.len() {
                    return (None, None);
                }
                let left: ZSV = ZSV::from(&self.data[..seperator]);
                let right: ZSV = ZSV::from(&self.data[seperator..]);
                return (Some(left), Some(right));
            } else {
                return (Some(self.clone()), None);
            }
        }

        /// Splits a ZSV into a tuple of `(a: Option<ZSV>, b: Option<ZSV>)` 
        /// where `Some(a)` is all data left of the index: `indx`, and 
        /// `Some(b)` is all data to the right of and including `indx`
        pub fn split_index(&self, indx: usize) -> (Option<ZSV>, Option<ZSV>) {
            if indx >= self.data.len() {
                return (None, None);
            }
            let left: ZSV = ZSV::from(&self.data[..indx]);
            let right: ZSV = ZSV::from(&self.data[indx..]);
            (Some(left), Some(right))
        }

        /// Splits a ZSV into a tuple of `(a: Option<ZSV>, b: Option<ZSV>)`
        /// This function is defined as `self.split_index(self.find_string(s))`
        pub fn split_string(&self, s: &ZSV) -> (Option<ZSV>, Option<ZSV>) {
            if let Some(indx) = self.find_string(s) {
                return self.split_index(indx);
            } else {
                return (Some(self.clone()), None);
            }
        }

        /// Returns an owned copy of the `String` 
        pub fn as_owned_string(&self) -> String {
            return self.data.to_owned();
        }
    }

    impl From<&str> for ZSV {
        fn from(data: &str) -> ZSV {
           ZSV {
               data: data.to_owned(),
           }
        }
    }
    impl From<String> for ZSV {
        fn from(data: String) -> ZSV {
           ZSV {
               data: data.clone(),
           } 
        }
    }
    impl From<&CString> for ZSV {
        fn from(data: &CString) -> ZSV {
            ZSV {
                data: data.to_str()
                    .expect("Invalid UTF-8 provided to ZSV::From<&CString>()")
                    .to_owned(),
            }
        }
    }
}
#[cfg(test)]
mod tests {

    use std::ffi::CString;
    use crate::z_sv::*;
    #[test]
    fn test_from_cstr() -> Result<(),()> {
        let j: CString = CString::new("Hello, world!").unwrap();
        let _f: ZSV = ZSV::from(&j);
        Ok(())
    }
    
    #[test]
    fn test_from_string() -> Result<(), ()> {
        let j: String = "The Game.".to_owned();
        let _f: ZSV = ZSV::from(j);
        Ok(())
    }

    #[test]
    fn test_find_string() -> Result<(), ()> {
        let corpus: ZSV = ZSV::from("Lorem ipsum dolor sit amet, consectetur 
                                    adipiscing elit. Duis vestibulum iaculis
                                    orci ut laoreet. Nulla hendrerit sed nisl
                                    nec tempor. Curabitur vitae tempus lorem,
                                    quis mollis velit. Morbi pellentesque 
                                    sodales turpis a vestibulum. Donec quis 
                                    justo sed odio sodales bibendum in 
                                    pellentesque erat. Pellentesque habitant 
                                    morbi tristique senectus et netus et 
                                    malesuada fames ac turpis egestas. 
                                    Vestibulum tincidunt libero non vestibulum 
                                    ultricies. Cras quis mi nunc. Donec vitae 
                                    tortor cursus lacus aliquam aliquam nec 
                                    gravida augue. Fusce iaculis est et est 
                                    fermentum fringilla. Sed vestibulum arcu 
                                    odio, at tristique nibh imperdiet vitae. 
                                    Curabitur sed velit molestie leo mollis 
                                    interdum sed id nunc. Sed sagittis 
                                    scelerisque tincidunt.");
        let text_to_find: ZSV = ZSV::from("et");
        let loc = corpus.find_string(&text_to_find);
        if let Some(loc) = loc {
            if loc != 24 {
                return Err(());
            } else {
                return Ok(());
            }
        } else {
            return Err(());
        }
    }

    #[test]
    fn test_rfind_string() -> Result<(), ()> {
        let corpus: ZSV = ZSV::from("Lorem ipsum dolor sit amet, consectetur 
                                    adipiscing elit. Duis vestibulum iaculis
                                    orci ut laoreet. Nulla hendrerit sed nisl
                                    nec tempor. Curabitur vitae tempus lorem,
                                    quis mollis velit. Morbi pellentesque 
                                    sodales turpis a vestibulum. Donec quis 
                                    justo sed odio sodales bibendum in 
                                    pellentesque erat. Pellentesque habitant 
                                    morbi tristique senectus et netus et 
                                    malesuada fames ac turpis egestas. 
                                    Vestibulum tincidunt libero non vestibulum 
                                    ultricies. Cras quis mi nunc. Donec vitae 
                                    tortor cursus lacus aliquam aliquam nec 
                                    gravida augue. Fusce iaculis est et est 
                                    fermentum fringilla. Sed vestibulum arcu 
                                    odio, at tristique nibh imperdiet vitae. 
                                    Curabitur sed velit molestie leo mollis 
                                    interdum sed id nunc. Sed sagittis 
                                    scelerisque tincidunt.");
        let text_to_find: ZSV = ZSV::from("et");
        let loc = corpus.rfind_string(&text_to_find);
        if let Some(loc) = loc {
            if loc != 1180 {
                return Err(());
            } else {
                return Ok(());
            }
        } else {
            return Err(());
        }
    }

    #[test]
    fn test_split_string() -> Result<(), ()> {
        let data: ZSV = ZSV::from("Quick brown fox != lazy dog");
        let search: ZSV = ZSV::from("!=");
        let ret = data.split_string(&search) == data.split_index(16);
        if ret {
            return Ok(());
        } else {
            return Err(());
        }
    }
}
