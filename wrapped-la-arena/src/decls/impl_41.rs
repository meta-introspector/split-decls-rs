macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < T > FusedIterator for IdxRange < T > { }
    };
}

impl_41!()