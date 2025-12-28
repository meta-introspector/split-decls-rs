macro_rules! deps {
    () => {
        StatusIter!();
    };
}

macro_rules! impl_756 {
    () => {
        deps!();
        impl < 'a > FusedIterator for StatusIter < 'a > { }
    };
}

impl_756!();