macro_rules! deps {
    () => {
        PropertyNamesLongBorrowed!();
        NamedEnumeratedProperty!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Copy for PropertyNamesLongBorrowed < '_ , T > { }
    };
}

impl_40!();