macro_rules! deps {
    () => {
        StrExt!();
        Suffixes!();
        Prefixes!();
        Substrings!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl StrExt for str { # [cfg (feature = "std-string")] fn rep (& self , n : usize) -> String { let mut s = String :: with_capacity (self . len () * n) ; s . extend ((0 .. n) . map (| _ | self)) ; s } # [cfg (feature = "std-string")] fn append (& self , s : & str) -> String { String :: from (self) + s } fn prefixes (& self) -> Prefixes { Prefixes { s : self , iter : self . char_indices () , } } fn suffixes (& self) -> Suffixes { Suffixes { s : self , iter : self . char_indices () , } } fn substrings (& self) -> Substrings { Substrings { iter : self . prefixes () . flat_map (str :: suffixes) , } } }
    };
}

impl_123!();