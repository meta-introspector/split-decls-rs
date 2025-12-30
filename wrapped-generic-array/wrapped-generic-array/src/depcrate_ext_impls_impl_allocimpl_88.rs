// Generated macro for impl_88 (impl)
macro_rules! Depcrate_ext_impls_impl_allocimpl_88 {
() => {
// Module: crate::ext_impls::impl_alloc
// Provides: {"impl_88"}
// Dependencies: {}
impl < T , N : ArrayLength > IntoIterator for Box < GenericArray < T , N > > { type IntoIter = alloc :: vec :: IntoIter < T > ; type Item = T ; fn into_iter (self) -> Self :: IntoIter { GenericArray :: into_vec (self) . into_iter () } }
};
}
