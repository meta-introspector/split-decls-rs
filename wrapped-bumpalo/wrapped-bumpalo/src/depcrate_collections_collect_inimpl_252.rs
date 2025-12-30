// Generated macro for impl_252 (impl)
macro_rules! Depcrate_collections_collect_inimpl_252 {
() => {
// Module: crate::collections::collect_in
// Provides: {"impl_252"}
// Dependencies: {}
impl < 'bump , T > FromIteratorIn < T > for Vec < 'bump , T > { type Alloc = & 'bump Bump ; fn from_iter_in < I > (iter : I , alloc : Self :: Alloc) -> Self where I : IntoIterator < Item = T > , { Vec :: from_iter_in (iter , alloc) } }
};
}
