// Generated macro for StoreIterableMut (trait)
macro_rules! Depcrate_storeStoreIterableMut {
() => {
// Module: crate::store
// Provides: {"StoreIterableMut"}
// Dependencies: {}
pub trait StoreIterableMut < 'a , K : 'a , V : 'a > : StoreMut < K , V > + StoreIterable < 'a , K , V > { type KeyValueIterMut : Iterator < Item = (& 'a K , & 'a mut V) > + DoubleEndedIterator + 'a ; # [doc = " Returns an iterator over key/value pairs, with a mutable value."] fn lm_iter_mut (& 'a mut self) -> Self :: KeyValueIterMut ; }
};
}
