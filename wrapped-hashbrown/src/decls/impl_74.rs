macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T > FusedIterator for RawIterRange < T > { }
    };
}

impl_74!()