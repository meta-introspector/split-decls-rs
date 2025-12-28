macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T > FusedIterator for IntoIter < T > { }
    };
}

impl_37!();