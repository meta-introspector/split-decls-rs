macro_rules! deps {
    () => {
        Splice!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < I : Iterator , const N : usize > ExactSizeIterator for Splice < '_ , I , N > { }
    };
}

impl_101!();