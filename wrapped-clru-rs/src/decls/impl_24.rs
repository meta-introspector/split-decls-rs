macro_rules! deps {
    () => {
        CLruCacheIntoIter!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < K : Clone + Eq + Hash , V , S : BuildHasher , W : WeightScale < K , V > > DoubleEndedIterator for CLruCacheIntoIter < K , V , S , W > { fn next_back (& mut self) -> Option < Self :: Item > { self . cache . pop_back () } }
    };
}

impl_24!()