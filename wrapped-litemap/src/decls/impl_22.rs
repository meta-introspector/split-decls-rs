macro_rules! deps {
    () => {
        StoreIterableMut!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a , S > LiteMap < K , V , S > where S : StoreIterableMut < 'a , K , V > , { # [doc = " Produce an ordered mutable iterator over key-value pairs"] pub fn iter_mut (& 'a mut self) -> impl DoubleEndedIterator < Item = (& 'a K , & 'a mut V) > { self . values . lm_iter_mut () } }
    };
}

impl_22!()