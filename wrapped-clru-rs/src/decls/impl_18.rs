macro_rules! deps {
    () => {
        CLruNode!();
        CLruCacheIterMut!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'a , K , V > Iterator for CLruCacheIterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (_ , CLruNode { key , value }) | (& * key , value)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_18!()