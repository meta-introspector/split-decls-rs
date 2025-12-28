macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < K , V > Clone for Keys < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Keys { inner : self . inner . clone () , } } }
    };
}

impl_268!();