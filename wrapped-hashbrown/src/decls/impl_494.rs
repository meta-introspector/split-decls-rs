macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for Iter < '_ , T > { fn len (& self) -> usize { self . inner . len () } }
    };
}

impl_494!()