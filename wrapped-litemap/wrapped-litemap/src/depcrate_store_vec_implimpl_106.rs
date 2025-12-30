// Generated macro for impl_106 (impl)
macro_rules! Depcrate_store_vec_implimpl_106 {
() => {
// Module: crate::store::vec_impl
// Provides: {"impl_106"}
// Dependencies: {}
impl < K : Ord , V > StoreFromIterable < K , V > for Vec < (K , V) > { fn lm_sort_from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self { let mut v = Self :: new () ; v . lm_extend (iter) ; v } }
};
}
