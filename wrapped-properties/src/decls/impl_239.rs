macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesLongBorrowed!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Copy for PropertyNamesLongBorrowed < '_ , T > { }
    };
}

impl_239!();