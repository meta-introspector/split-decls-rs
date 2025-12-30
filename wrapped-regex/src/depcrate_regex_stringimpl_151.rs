// Generated macro for impl_151 (impl)
macro_rules! Depcrate_regex_stringimpl_151 {
() => {
// Module: crate::regex::string
// Provides: {"impl_151"}
// Dependencies: {}
impl < 's > Replacer for NoExpand < 's > { fn replace_append (& mut self , _ : & Captures < '_ > , dst : & mut String) { dst . push_str (self . 0) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { Some (Cow :: Borrowed (self . 0)) } }
};
}
