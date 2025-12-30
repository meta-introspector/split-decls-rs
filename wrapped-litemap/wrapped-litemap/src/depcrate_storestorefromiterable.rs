// Generated macro for StoreFromIterable (trait)
macro_rules! Depcrate_storeStoreFromIterable {
() => {
// Module: crate::store
// Provides: {"StoreFromIterable"}
// Dependencies: {}
pub trait StoreFromIterable < K , V > : Store < K , V > { # [doc = " Create a sorted store from `iter`."] fn lm_sort_from_iter < I : IntoIterator < Item = (K , V) > > (iter : I) -> Self ; }
};
}
