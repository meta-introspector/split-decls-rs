macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < K , V , A : Allocator > ExactSizeIterator for IntoIter < K , V , A > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_308!();