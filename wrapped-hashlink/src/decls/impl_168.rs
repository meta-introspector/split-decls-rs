macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl < K > ExactSizeIterator for Drain < '_ , K > { }
    };
}

impl_168!();