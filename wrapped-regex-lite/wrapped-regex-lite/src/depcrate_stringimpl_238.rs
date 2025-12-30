// Generated macro for impl_238 (impl)
macro_rules! Depcrate_stringimpl_238 {
() => {
// Module: crate::string
// Provides: {"impl_238"}
// Dependencies: {}
impl < 't > Replacer for NoExpand < 't > { fn replace_append (& mut self , _ : & Captures < '_ > , dst : & mut String) { dst . push_str (self . 0) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { Some (Cow :: Borrowed (self . 0)) } }
};
}
