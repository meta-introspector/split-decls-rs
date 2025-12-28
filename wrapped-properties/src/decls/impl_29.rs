macro_rules! deps {
    () => {
        PropertyParserBorrowed!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < T > Copy for PropertyParserBorrowed < '_ , T > { }
    };
}

impl_29!();