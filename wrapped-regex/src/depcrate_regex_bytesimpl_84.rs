// Generated macro for impl_84 (impl)
macro_rules! Depcrate_regex_bytesimpl_84 {
() => {
// Module: crate::regex::bytes
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a > Replacer for & 'a Vec < u8 > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (* self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
};
}
