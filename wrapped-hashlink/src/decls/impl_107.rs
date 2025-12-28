macro_rules! deps {
    () => {
        ValueLinks!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < K , V > Clone for ValueLinks < K , V > { # [inline] fn clone (& self) -> Self { * self } }
    };
}

impl_107!();