macro_rules! deps {
    () => {
        IntoIter!();
        LinearMap!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < K , V , const N : usize > IntoIterator for LinearMap < K , V , N > where K : Eq , { type Item = (K , V) ; type IntoIter = IntoIter < K , V , N > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { inner : self . buffer . into_iter () , } } }
    };
}

impl_181!()