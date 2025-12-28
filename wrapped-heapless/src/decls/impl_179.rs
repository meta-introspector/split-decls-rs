macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < K , V , const N : usize > FusedIterator for IntoIter < K , V , N > where K : Eq { }
    };
}

impl_179!()