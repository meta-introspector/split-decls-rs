macro_rules! deps {
    () => {
        IdxRange!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T > Eq for IdxRange < T > { }
    };
}

impl_45!()