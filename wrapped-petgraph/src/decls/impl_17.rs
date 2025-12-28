macro_rules! deps {
    () => {
        MaxScored!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < K : PartialOrd , T > Eq for MaxScored < K , T > { }
    };
}

impl_17!()