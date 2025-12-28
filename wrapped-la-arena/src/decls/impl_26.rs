macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T > Eq for IdxRange < T > { }
    };
}

impl_26!()