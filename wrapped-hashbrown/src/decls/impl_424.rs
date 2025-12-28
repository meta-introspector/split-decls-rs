macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl < K > Default for Iter < '_ , K > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Iter { iter : Default :: default () , } } }
    };
}

impl_424!()