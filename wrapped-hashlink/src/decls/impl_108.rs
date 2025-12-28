macro_rules! deps {
    () => {
        ValueLinks!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < K , V > Copy for ValueLinks < K , V > { }
    };
}

impl_108!()