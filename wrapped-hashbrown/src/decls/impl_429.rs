macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl < K , A : Allocator > Default for IntoIter < K , A > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { IntoIter { iter : Default :: default () , } } }
    };
}

impl_429!();