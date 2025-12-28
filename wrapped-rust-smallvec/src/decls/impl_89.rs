macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < T , const N : usize > ExactSizeIterator for Drain < '_ , T , N > { # [inline] fn len (& self) -> usize { self . iter . len () } }
    };
}

impl_89!();