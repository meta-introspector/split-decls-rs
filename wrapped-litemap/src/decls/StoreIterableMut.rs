macro_rules! deps {
    () => {
        StoreIterable!();
        StoreMut!();
    };
}

macro_rules! StoreIterableMut {
    () => {
        deps!();
        pub trait StoreIterableMut < 'a , K : 'a , V : 'a > : StoreMut < K , V > + StoreIterable < 'a , K , V > { type KeyValueIterMut : Iterator < Item = (& 'a K , & 'a mut V) > + DoubleEndedIterator + 'a ; # [doc = " Returns an iterator over key/value pairs, with a mutable value."] fn lm_iter_mut (& 'a mut self) -> Self :: KeyValueIterMut ; }
    };
}

StoreIterableMut!()