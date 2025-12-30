// Generated macro for impl_91 (impl)
macro_rules! Depcrate_store_slice_implimpl_91 {
() => {
// Module: crate::store::slice_impl
// Provides: {"impl_91"}
// Dependencies: {}
impl < K , V > StoreSlice < K , V > for & [(K , V)] { type Slice = [(K , V)] ; fn lm_get_range (& self , range : Range < usize >) -> Option < & Self :: Slice > { self . get (range) } }
};
}
