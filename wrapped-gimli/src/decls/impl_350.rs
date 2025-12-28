macro_rules! deps {
    () => {
        Attributes!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl Eq for Attributes { }
    };
}

impl_350!()