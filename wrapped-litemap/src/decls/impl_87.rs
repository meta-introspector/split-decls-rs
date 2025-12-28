macro_rules! deps {
    () => {
        StoreFromIterator!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < K , V > StoreFromIterator < K , V > for Vec < (K , V) > { }
    };
}

impl_87!();