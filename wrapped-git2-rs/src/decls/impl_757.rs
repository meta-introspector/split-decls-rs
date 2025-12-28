macro_rules! deps {
    () => {
        StatusIter!();
    };
}

macro_rules! impl_757 {
    () => {
        deps!();
        impl < 'a > ExactSizeIterator for StatusIter < 'a > { }
    };
}

impl_757!();