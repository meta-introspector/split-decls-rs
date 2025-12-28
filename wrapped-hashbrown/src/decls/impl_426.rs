macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl < K > ExactSizeIterator for Iter < '_ , K > { # [cfg_attr (feature = "inline-more" , inline)] fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_426!();