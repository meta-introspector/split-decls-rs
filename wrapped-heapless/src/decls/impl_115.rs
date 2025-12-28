macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < K , V , const N : usize > FusedIterator for IntoIter < K , V , N > { }
    };
}

impl_115!()