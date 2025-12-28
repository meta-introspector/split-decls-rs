macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Clone for PropertyNamesShortBorrowed < '_ , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_48!()