macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < K , V , P > Clone for Key < K , V , P > { fn clone (& self) -> Key < K , V , P > { * self } }
    };
}

impl_132!()