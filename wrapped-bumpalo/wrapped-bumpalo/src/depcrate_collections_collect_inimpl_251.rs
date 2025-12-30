// Generated macro for impl_251 (impl)
macro_rules! Depcrate_collections_collect_inimpl_251 {
() => {
// Module: crate::collections::collect_in
// Provides: {"impl_251"}
// Dependencies: {}
# [cfg (feature = "boxed")] impl < 'bump , T > FromIteratorIn < T > for Box < 'bump , [T] > { type Alloc = & 'bump Bump ; fn from_iter_in < I > (iter : I , alloc : Self :: Alloc) -> Self where I : IntoIterator < Item = T > , { Box :: from_iter_in (iter , alloc) } }
};
}
