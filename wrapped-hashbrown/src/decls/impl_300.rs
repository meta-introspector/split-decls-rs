macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl < K , V > FusedIterator for Iter < '_ , K , V > { }
    };
}

impl_300!()