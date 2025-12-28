macro_rules! deps {
    () => {
        MinScored!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < K : PartialOrd , T > Eq for MinScored < K , T > { }
    };
}

impl_12!();