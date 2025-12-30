// Generated macro for impl_229 (impl)
macro_rules! Depcrate_stringimpl_229 {
() => {
// Module: crate::string
// Provides: {"impl_229"}
// Dependencies: {}
impl < 'a > Replacer for & 'a str { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { caps . expand (* self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
};
}
