macro_rules! deps {
    () => {
        IntoValues!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl < K , V , A : Allocator > Default for IntoValues < K , V , A > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self { inner : Default :: default () , } } }
    };
}

impl_262!()