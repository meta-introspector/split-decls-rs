// Generated macro for impl_205 (impl)
macro_rules! Depcrate_collections_stringimpl_205 {
() => {
// Module: crate::collections::string
// Provides: {"impl_205"}
// Dependencies: {}
impl < 'a , 'bump > Extend < Cow < 'a , str > > for String < 'bump > { fn extend < I : IntoIterator < Item = Cow < 'a , str > > > (& mut self , iter : I) { for s in iter { self . push_str (& s) } } }
};
}
