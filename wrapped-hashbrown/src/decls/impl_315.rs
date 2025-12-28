macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < K , V > Default for Values < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , } } }
    };
}

impl_315!()