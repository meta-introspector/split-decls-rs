macro_rules! deps {
    () => {
        Refspecs!();
    };
}

macro_rules! impl_651 {
    () => {
        deps!();
        impl < 'repo > ExactSizeIterator for Refspecs < 'repo > { }
    };
}

impl_651!();