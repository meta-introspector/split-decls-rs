macro_rules! deps {
    () => {
        RepeatN!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl < A > ExactSizeIterator for RepeatN < A > where A : Clone { }
    };
}

impl_468!()