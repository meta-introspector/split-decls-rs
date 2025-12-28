macro_rules! deps {
    () => {
        IntoOnes!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl FusedIterator for IntoOnes { }
    };
}

impl_59!()