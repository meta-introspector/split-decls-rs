// Generated macro for impl_145 (impl)
macro_rules! Depcrate_regex_stringimpl_145 {
() => {
// Module: crate::regex::string
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'a > Replacer for Cow < 'a , str > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_ref () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
};
}
