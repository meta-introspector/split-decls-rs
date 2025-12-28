macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl < K , V > FusedIterator for Values < '_ , K , V > { }
    };
}

impl_318!();