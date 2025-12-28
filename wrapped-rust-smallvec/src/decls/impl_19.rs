macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T , const N : usize > ExactSizeIterator for Drain < '_ , T , N > { # [inline] fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_19!()