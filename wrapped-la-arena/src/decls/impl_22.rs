macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T > FusedIterator for IdxRange < T > { }
    };
}

impl_22!()