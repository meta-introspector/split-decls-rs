macro_rules! deps {
    () => {
        IntoOnes!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl FusedIterator for IntoOnes { }
    };
}

impl_128!()