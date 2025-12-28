macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for IdxRange < T > { }
    };
}

impl_21!()