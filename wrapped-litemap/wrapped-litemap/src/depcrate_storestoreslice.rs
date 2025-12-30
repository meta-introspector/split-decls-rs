// Generated macro for StoreSlice (trait)
macro_rules! Depcrate_storeStoreSlice {
() => {
// Module: crate::store
// Provides: {"StoreSlice"}
// Dependencies: {}
pub trait StoreSlice < K : ? Sized , V : ? Sized > : Store < K , V > { type Slice : ? Sized ; fn lm_get_range (& self , range : Range < usize >) -> Option < & Self :: Slice > ; }
};
}
