macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < K , V > FusedIterator for ValuesMut < '_ , K , V > { }
    };
}

impl_322!();