// Generated macro for impl_90 (impl)
macro_rules! Depcrate_regex_bytesimpl_90 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_90"}
// Dependencies: {}
impl < 'a , R : Replacer + ? Sized + 'a > Replacer for ReplacerRef < 'a , R > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { self . 0 . replace_append (caps , dst) } fn no_expansion < 'r > (& 'r mut self) -> Option < Cow < 'r , [u8] > > { self . 0 . no_expansion () } }
};
}
