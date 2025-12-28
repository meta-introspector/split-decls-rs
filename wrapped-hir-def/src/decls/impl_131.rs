macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < K , V , P > Copy for Key < K , V , P > { }
    };
}

impl_131!();