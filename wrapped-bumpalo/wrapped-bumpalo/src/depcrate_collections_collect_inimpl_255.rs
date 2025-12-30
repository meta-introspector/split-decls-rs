// Generated macro for impl_255 (impl)
macro_rules! Depcrate_collections_collect_inimpl_255 {
() => {
// Module: crate::collections::collect_in
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'bump > FromIteratorIn < char > for String < 'bump > { type Alloc = & 'bump Bump ; fn from_iter_in < I > (iter : I , alloc : Self :: Alloc) -> Self where I : IntoIterator < Item = char > , { String :: from_iter_in (iter , alloc) } }
};
}
