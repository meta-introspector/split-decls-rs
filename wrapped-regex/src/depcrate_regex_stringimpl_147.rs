// Generated macro for impl_147 (impl)
macro_rules! Depcrate_regex_stringimpl_147 {
() => {
// Module: crate::regex::string
// Provides: {"impl_147"}
// Dependencies: {}
impl < F , T > Replacer for F where F : FnMut (& Captures < '_ >) -> T , T : AsRef < str > , { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { dst . push_str ((* self) (caps) . as_ref ()) ; } }
};
}
