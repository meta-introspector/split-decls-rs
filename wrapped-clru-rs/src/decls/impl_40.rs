macro_rules! deps {
    () => {
        CLruCacheIterMut!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < K , V > ExactSizeIterator for CLruCacheIterMut < '_ , K , V > { fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_40!()