macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < K , V > Clone for Values < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Values { inner : self . inner . clone () , } } }
    };
}

impl_271!()