// Generated macro for impl_146 (impl)
macro_rules! Depcrate_regex_stringimpl_146 {
() => {
// Module: crate::regex::string
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'a > Replacer for & 'a Cow < 'a , str > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_ref () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
};
}
