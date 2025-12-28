macro_rules! deps {
    () => {
        ExactlyOneError!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < I > ExactSizeIterator for ExactlyOneError < I > where I : ExactSizeIterator { }
    };
}

impl_220!();