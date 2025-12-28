macro_rules! deps {
    () => {
        WeightScale!();
        CLruCacheIntoIter!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < K : Clone + Eq + Hash , V , S : BuildHasher , W : WeightScale < K , V > > ExactSizeIterator for CLruCacheIntoIter < K , V , S , W > { fn len (& self) -> usize { self . size_hint () . 0 } }
    };
}

impl_45!();