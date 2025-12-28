macro_rules! deps {
    () => {
        RawIter!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < T > FusedIterator for RawIter < T > { }
    };
}

impl_81!();