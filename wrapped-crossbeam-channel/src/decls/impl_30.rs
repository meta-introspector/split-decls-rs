macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T > FusedIterator for Iter < '_ , T > { }
    };
}

impl_30!();