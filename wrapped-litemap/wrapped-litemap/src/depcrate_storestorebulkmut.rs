// Generated macro for StoreBulkMut (trait)
macro_rules! Depcrate_storeStoreBulkMut {
() => {
// Module: crate::store
// Provides: {"StoreBulkMut"}
// Dependencies: {}
pub trait StoreBulkMut < K , V > : StoreMut < K , V > { # [doc = " Retains items satisfying a predicate in this store."] fn lm_retain < F > (& mut self , predicate : F) where F : FnMut (& K , & V) -> bool ; # [doc = " Extends this store with items from an iterator."] fn lm_extend < I > (& mut self , other : I) where I : IntoIterator < Item = (K , V) > ; }
};
}
