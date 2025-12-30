// Generated macro for impl_203 (impl)
macro_rules! Depcrate_collections_stringimpl_203 {
() => {
// Module: crate::collections::string
// Provides: {"impl_203"}
// Dependencies: {}
impl < 'bump > Extend < String < 'bump > > for String < 'bump > { fn extend < I : IntoIterator < Item = String < 'bump > > > (& mut self , iter : I) { for s in iter { self . push_str (& s) } } }
};
}
