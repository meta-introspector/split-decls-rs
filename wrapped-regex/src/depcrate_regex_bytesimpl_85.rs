// Generated macro for impl_85 (impl)
macro_rules! Depcrate_regex_bytesimpl_85 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_85"}
// Dependencies: {}
impl Replacer for Vec < u8 > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
};
}
