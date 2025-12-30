// Generated macro for StoreIterable (trait)
macro_rules! Depcrate_storeStoreIterable {
() => {
// Module: crate::store
// Provides: {"StoreIterable"}
// Dependencies: {}
# [doc = " Iterator methods for the LiteMap store."] pub trait StoreIterable < 'a , K : 'a + ? Sized , V : 'a + ? Sized > : Store < K , V > { type KeyValueIter : Iterator < Item = (& 'a K , & 'a V) > + DoubleEndedIterator + 'a ; # [doc = " Returns an iterator over key/value pairs."] fn lm_iter (& 'a self) -> Self :: KeyValueIter ; }
};
}
