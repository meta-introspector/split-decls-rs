macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl Eq for Either { }
    };
}

impl_82!();