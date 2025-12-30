// Generated macro for impl_230 (impl)
macro_rules! Depcrate_stringimpl_230 {
() => {
// Module: crate::string
// Provides: {"impl_230"}
// Dependencies: {}
impl < 'a > Replacer for & 'a String { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_str () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
};
}
