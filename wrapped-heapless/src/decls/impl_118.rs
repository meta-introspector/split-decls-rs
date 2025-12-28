macro_rules! deps {
    () => {
        Iter!();
        IndexMap!();
        IntoIter!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'a , K , V , S , const N : usize > IntoIterator for & 'a IndexMap < K , V , S , N > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_118!();