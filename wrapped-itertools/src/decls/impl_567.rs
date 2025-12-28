macro_rules! deps {
    () => {
        WithPosition!();
    };
}

macro_rules! impl_567 {
    () => {
        deps!();
        impl < I > ExactSizeIterator for WithPosition < I > where I : ExactSizeIterator { }
    };
}

impl_567!()