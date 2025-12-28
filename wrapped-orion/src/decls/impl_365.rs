macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl Eq for FieldElement { }
    };
}

impl_365!();