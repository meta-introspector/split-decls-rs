macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < K , A : Allocator > ExactSizeIterator for IntoIter < K , A > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_431!()