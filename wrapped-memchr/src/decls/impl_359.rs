macro_rules! deps {
    () => {
        Pre!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < 'a > Pre < 'a > { # [doc = " Call this prefilter on the given haystack with the given needle."] # [inline] pub (crate) fn find (& mut self , haystack : & [u8]) -> Option < usize > { let result = self . prestrat . find (haystack) ; self . prestate . update (result . unwrap_or (haystack . len ())) ; result } # [doc = " Return true if and only if this prefilter should be used."] # [inline] pub (crate) fn is_effective (& mut self) -> bool { self . prestate . is_effective () } }
    };
}

impl_359!();