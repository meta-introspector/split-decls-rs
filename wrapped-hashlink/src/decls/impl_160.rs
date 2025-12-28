macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < K > ExactSizeIterator for Iter < '_ , K > { }
    };
}

impl_160!();