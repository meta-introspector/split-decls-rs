macro_rules! deps {
    () => {
        IterProducer!();
        IntoIter!();
    };
}

macro_rules! impl_1057 {
    () => {
        deps!();
        impl < T > IntoIterator for IterProducer < T > where Range < T > : Iterator , { type Item = < Range < T > as Iterator > :: Item ; type IntoIter = Range < T > ; fn into_iter (self) -> Self :: IntoIter { self . range } }
    };
}

impl_1057!();