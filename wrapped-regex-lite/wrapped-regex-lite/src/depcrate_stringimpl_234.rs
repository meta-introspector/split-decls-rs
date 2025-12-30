// Generated macro for impl_234 (impl)
macro_rules! Depcrate_stringimpl_234 {
() => {
// Module: crate::string
// Provides: {"impl_234"}
// Dependencies: {}
impl < F , T > Replacer for F where F : FnMut (& Captures < '_ >) -> T , T : AsRef < str > , { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { dst . push_str ((* self) (caps) . as_ref ()) ; } }
};
}
