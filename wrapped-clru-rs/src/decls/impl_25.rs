macro_rules! deps {
    () => {
        CLruCacheIntoIter!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < K : Clone + Eq + Hash , V , S : BuildHasher , W : WeightScale < K , V > > ExactSizeIterator for CLruCacheIntoIter < K , V , S , W > { fn len (& self) -> usize { self . size_hint () . 0 } }
    };
}

impl_25!()