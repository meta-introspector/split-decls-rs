macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < K , V , const N : usize > ExactSizeIterator for IntoIter < K , V , N > where K : Eq , { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_180!()