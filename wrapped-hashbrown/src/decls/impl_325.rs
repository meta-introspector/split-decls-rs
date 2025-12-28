macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl < K , V , A : Allocator > ExactSizeIterator for Drain < '_ , K , V , A > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_325!()