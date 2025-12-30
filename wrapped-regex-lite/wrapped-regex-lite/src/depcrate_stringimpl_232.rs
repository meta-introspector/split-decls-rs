// Generated macro for impl_232 (impl)
macro_rules! Depcrate_stringimpl_232 {
() => {
// Module: crate::string
// Provides: {"impl_232"}
// Dependencies: {}
impl < 'a > Replacer for Cow < 'a , str > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_ref () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
};
}
