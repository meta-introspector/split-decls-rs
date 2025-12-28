macro_rules! deps {
    () => {
        IntoValues!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < K , V , A : Allocator > ExactSizeIterator for IntoValues < K , V , A > { # [inline] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_264!()