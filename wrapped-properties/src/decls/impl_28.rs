macro_rules! deps {
    () => {
        PropertyParserBorrowed!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < T > Clone for PropertyParserBorrowed < '_ , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_28!()