macro_rules! deps {
    () => {
        Masks!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl FusedIterator for Masks { }
    };
}

impl_110!();