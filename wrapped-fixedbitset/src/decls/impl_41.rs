macro_rules! deps {
    () => {
        Masks!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl FusedIterator for Masks { }
    };
}

impl_41!()