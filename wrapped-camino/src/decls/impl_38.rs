macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl FusedIterator for Iter < '_ > { }
    };
}

impl_38!()