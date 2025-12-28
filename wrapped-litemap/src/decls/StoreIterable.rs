macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! StoreIterable {
    () => {
        deps!();
        # [doc = " Iterator methods for the LiteMap store."] pub trait StoreIterable < 'a , K : 'a + ? Sized , V : 'a + ? Sized > : Store < K , V > { type KeyValueIter : Iterator < Item = (& 'a K , & 'a V) > + DoubleEndedIterator + 'a ; # [doc = " Returns an iterator over key/value pairs."] fn lm_iter (& 'a self) -> Self :: KeyValueIter ; }
    };
}

StoreIterable!();