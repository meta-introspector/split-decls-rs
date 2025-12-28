macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Copy for PropertyNamesShortBorrowed < '_ , T > { }
    };
}

impl_49!();