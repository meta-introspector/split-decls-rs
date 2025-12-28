macro_rules! deps {
    () => {
        Flags!();
        IterDefinedNames!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < B : Flags > IterDefinedNames < B > { pub (crate) fn new () -> Self { IterDefinedNames { flags : B :: FLAGS , idx : 0 , } } }
    };
}

impl_9!();