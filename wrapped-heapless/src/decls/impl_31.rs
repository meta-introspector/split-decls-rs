macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > FusedIterator for Iter < '_ , T > { }
    };
}

impl_31!()