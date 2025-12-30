// Generated macro for impl_86 (impl)
macro_rules! Depcrate_regex_bytesimpl_86 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a > Replacer for Cow < 'a , [u8] > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (self . as_ref () , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
};
}
