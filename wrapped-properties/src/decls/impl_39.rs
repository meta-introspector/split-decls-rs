macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesLongBorrowed!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Clone for PropertyNamesLongBorrowed < '_ , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_39!()