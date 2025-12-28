macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl Eq for IdentUnraw { }
    };
}

impl_97!()