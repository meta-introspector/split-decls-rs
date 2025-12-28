macro_rules! deps {
    () => {
        StoreIterable!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < 'a , K , V , S > IntoIterator for & 'a LiteMap < K , V , S > where S : StoreIterable < 'a , K , V > , { type Item = (& 'a K , & 'a V) ; type IntoIter = S :: KeyValueIter ; fn into_iter (self) -> Self :: IntoIter { self . values . lm_iter () } }
    };
}

impl_24!();