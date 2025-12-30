// Generated macro for impl_201 (impl)
macro_rules! Depcrate_collections_stringimpl_201 {
() => {
// Module: crate::collections::string
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'a , 'bump > Extend < & 'a char > for String < 'bump > { fn extend < I : IntoIterator < Item = & 'a char > > (& mut self , iter : I) { self . extend (iter . into_iter () . cloned ()) ; } }
};
}
