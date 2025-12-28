macro_rules! deps {
    () => {
        RawIter!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for RawIter < T > { }
    };
}

impl_80!()