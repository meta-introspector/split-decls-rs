// Generated macro for impl_87 (impl)
macro_rules! Depcrate_regex_bytesimpl_87 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a > Replacer for & 'a Cow < 'a , [u8] > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (self . as_ref () , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
};
}
