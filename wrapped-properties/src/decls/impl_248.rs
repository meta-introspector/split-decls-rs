macro_rules! deps {
    () => {
        NamedEnumeratedProperty!();
        PropertyNamesShortBorrowed!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < T : NamedEnumeratedProperty > Copy for PropertyNamesShortBorrowed < '_ , T > { }
    };
}

impl_248!();