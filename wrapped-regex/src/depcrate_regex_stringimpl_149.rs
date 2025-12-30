// Generated macro for impl_149 (impl)
macro_rules! Depcrate_regex_stringimpl_149 {
() => {
// Module: crate::regex::string
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'a , R : Replacer + ? Sized + 'a > Replacer for ReplacerRef < 'a , R > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . 0 . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { self . 0 . no_expansion () } }
};
}
