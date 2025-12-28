macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < K , V , const N : usize > ExactSizeIterator for IntoIter < K , V , N > { fn len (& self) -> usize { self . entries . len () } }
    };
}

impl_116!()