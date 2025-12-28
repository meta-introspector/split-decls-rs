macro_rules! deps {
    () => {
        ValuesMut!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl < K , V > Default for ValuesMut < '_ , K , V > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , } } }
    };
}

impl_319!()