macro_rules! deps {
    () => {
        CLruCacheIter!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for CLruCacheIter < '_ , K , V > { fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_35!()