macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl < K , V , A : Allocator > Default for IntoIter < K , V , A > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , } } }
    };
}

impl_306!()