macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl < K , V > FusedIterator for IterMut < '_ , K , V > { }
    };
}

impl_304!()