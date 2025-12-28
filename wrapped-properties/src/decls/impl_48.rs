macro_rules! deps {
    () => {
        PropertyNamesShortBorrowed!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Clone for PropertyNamesShortBorrowed < '_ , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_48!();