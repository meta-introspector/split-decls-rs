// Generated macro for impl_102 (impl)
macro_rules! Depcrate_store_vec_implimpl_102 {
() => {
// Module: crate::store::vec_impl
// Provides: {"impl_102"}
// Dependencies: {}
impl < K , V > StoreSlice < K , V > for Vec < (K , V) > { type Slice = [(K , V)] ; fn lm_get_range (& self , range : Range < usize >) -> Option < & Self :: Slice > { self . get (range) } }
};
}
