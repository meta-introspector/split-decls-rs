macro_rules! deps {
    () => {
        IntoKeys!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < K , V , A : Allocator > ExactSizeIterator for IntoKeys < K , V , A > { # [inline] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_258!();