macro_rules! deps {
    () => {
        TokenAtOffset!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for TokenAtOffset < T > { }
    };
}

impl_110!()