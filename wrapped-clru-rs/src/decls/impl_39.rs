macro_rules! deps {
    () => {
        CLruNode!();
        CLruCacheIterMut!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < K , V > DoubleEndedIterator for CLruCacheIterMut < '_ , K , V > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (_ , CLruNode { key , value }) | (& * key , value)) } }
    };
}

impl_39!()