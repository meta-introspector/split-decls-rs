// Generated macro for impl_144 (impl)
macro_rules! Depcrate_regex_stringimpl_144 {
() => {
// Module: crate::regex::string
// Provides: {"impl_144"}
// Dependencies: {}
impl Replacer for String { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_str () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
};
}
