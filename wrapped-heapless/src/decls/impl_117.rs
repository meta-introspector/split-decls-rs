macro_rules! deps {
    () => {
        IndexMap!();
        IntoIter!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > IntoIterator for IndexMap < K , V , S , N > { type Item = (K , V) ; type IntoIter = IntoIter < K , V , N > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { entries : self . core . entries , } } }
    };
}

impl_117!();