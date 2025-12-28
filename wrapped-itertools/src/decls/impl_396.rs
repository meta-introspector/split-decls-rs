macro_rules! deps {
    () => {
        PeekNth!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        impl < I > ExactSizeIterator for PeekNth < I > where I : ExactSizeIterator { }
    };
}

impl_396!();