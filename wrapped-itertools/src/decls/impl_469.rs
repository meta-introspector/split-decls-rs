macro_rules! deps {
    () => {
        RepeatN!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < A > FusedIterator for RepeatN < A > where A : Clone { }
    };
}

impl_469!()