macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl FusedIterator for Iter < '_ > { }
    };
}

impl_55!()