macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl < K , V > Default for Keys < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , } } }
    };
}

impl_311!()