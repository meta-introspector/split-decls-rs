macro_rules! deps {
    () => {
        CLruCacheIter!();
        CLruNode!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for CLruCacheIter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (_ , CLruNode { key , value }) | (key . borrow () , value)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_33!();