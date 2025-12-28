macro_rules! deps {
    () => {
        StoreIterableMut!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoIterator for & 'a mut LiteMap < K , V , S > where S : StoreIterableMut < 'a , K , V > , { type Item = (& 'a K , & 'a mut V) ; type IntoIter = S :: KeyValueIterMut ; fn into_iter (self) -> Self :: IntoIter { self . values . lm_iter_mut () } }
    };
}

impl_25!()