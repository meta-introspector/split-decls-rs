// Generated macro for impl_166 (impl)
macro_rules! Depcrate_stringimpl_166 {
() => {
// Module: crate::string
// Provides: {"impl_166"}
// Dependencies: {}
impl StrExt for str { # [cfg (feature = "std-string")] fn rep (& self , n : usize) -> String { let mut s = String :: with_capacity (self . len () * n) ; s . extend ((0 .. n) . map (| _ | self)) ; s } # [cfg (feature = "std-string")] fn append (& self , s : & str) -> String { String :: from (self) + s } fn prefixes (& self) -> Prefixes { Prefixes { s : self , iter : self . char_indices () , } } fn suffixes (& self) -> Suffixes { Suffixes { s : self , iter : self . char_indices () , } } fn substrings (& self) -> Substrings { Substrings { iter : self . prefixes () . flat_map (str :: suffixes) , } } }
};
}
