// Generated macro for impl_81 (impl)
macro_rules! Depcrate_regex_bytesimpl_81 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a , const N : usize > Replacer for & 'a [u8 ; N] { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (& * * self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
};
}
