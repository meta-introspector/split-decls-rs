macro_rules! deps {
    () => {
        Deltas!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < 'diff > ExactSizeIterator for Deltas < 'diff > { }
    };
}

impl_341!();