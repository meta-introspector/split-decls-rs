macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IdxRange < T > { }
    };
}

impl_40!()