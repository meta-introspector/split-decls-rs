macro_rules! deps {
    () => {
        MultiPeek!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        impl < I > ExactSizeIterator for MultiPeek < I > where I : ExactSizeIterator { }
    };
}

impl_374!()