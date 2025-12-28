macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < K , V > Clone for Iter < '_ , K , V > { # [inline] fn clone (& self) -> Self { Iter { .. * self } } }
    };
}

impl_66!();