macro_rules! deps {
    () => {
        BodyIter!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl FusedIterator for BodyIter < '_ > { }
    };
}

impl_82!();