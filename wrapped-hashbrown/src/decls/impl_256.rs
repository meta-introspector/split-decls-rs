macro_rules! deps {
    () => {
        IntoKeys!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl < K , V , A : Allocator > Default for IntoKeys < K , V , A > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , } } }
    };
}

impl_256!();