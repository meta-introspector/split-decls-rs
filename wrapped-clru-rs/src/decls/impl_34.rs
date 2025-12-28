macro_rules! deps {
    () => {
        CLruCacheIter!();
        CLruNode!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < K , V > DoubleEndedIterator for CLruCacheIter < '_ , K , V > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (_ , CLruNode { key , value }) | (key . borrow () , value)) } }
    };
}

impl_34!()