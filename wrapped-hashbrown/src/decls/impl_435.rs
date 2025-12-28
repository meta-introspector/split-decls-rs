macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < K , A : Allocator > ExactSizeIterator for Drain < '_ , K , A > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_435!();