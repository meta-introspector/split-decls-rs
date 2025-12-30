// Generated macro for impl_82 (impl)
macro_rules! Depcrate_regex_bytesimpl_82 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_82"}
// Dependencies: {}
impl < const N : usize > Replacer for [u8 ; N] { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (& * self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
};
}
