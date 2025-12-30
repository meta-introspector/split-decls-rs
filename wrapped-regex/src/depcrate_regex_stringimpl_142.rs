// Generated macro for impl_142 (impl)
macro_rules! Depcrate_regex_stringimpl_142 {
() => {
// Module: crate::regex::string
// Provides: {"impl_142"}
// Dependencies: {}
impl < 'a > Replacer for & 'a str { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { caps . expand (* self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
};
}
