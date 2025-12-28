macro_rules! deps {
    () => {
        TraitKind!();
        Error!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < E > TraitKind for E where E : Into < Error > { }
    };
}

impl_97!();