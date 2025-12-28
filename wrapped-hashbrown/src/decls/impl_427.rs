macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl < K > FusedIterator for Iter < '_ , K > { }
    };
}

impl_427!();