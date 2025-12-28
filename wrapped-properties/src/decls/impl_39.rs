macro_rules! deps {
    () => {
        PropertyNamesLongBorrowed!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Clone for PropertyNamesLongBorrowed < '_ , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_39!();