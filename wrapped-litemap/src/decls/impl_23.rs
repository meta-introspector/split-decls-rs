macro_rules! deps {
    () => {
        StoreIntoIterator!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < K , V , S > IntoIterator for LiteMap < K , V , S > where S : StoreIntoIterator < K , V > , { type Item = (K , V) ; type IntoIter = S :: KeyValueIntoIter ; fn into_iter (self) -> Self :: IntoIter { self . values . lm_into_iter () } }
    };
}

impl_23!();