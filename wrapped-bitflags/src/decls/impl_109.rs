macro_rules! deps {
    () => {
        IterDefinedNames!();
        Flags!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < B : Flags > IterDefinedNames < B > { pub (crate) fn new () -> Self { IterDefinedNames { flags : B :: FLAGS , idx : 0 , } } }
    };
}

impl_109!();