// Generated macro for impl_202 (impl)
macro_rules! Depcrate_collections_stringimpl_202 {
() => {
// Module: crate::collections::string
// Provides: {"impl_202"}
// Dependencies: {}
impl < 'a , 'bump > Extend < & 'a str > for String < 'bump > { fn extend < I : IntoIterator < Item = & 'a str > > (& mut self , iter : I) { for s in iter { self . push_str (s) } } }
};
}
