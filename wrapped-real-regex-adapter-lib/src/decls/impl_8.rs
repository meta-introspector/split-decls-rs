macro_rules! deps {
    () => {
        DummyRegexCaptures!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 't > RegexCaptures for DummyRegexCaptures { fn get (& self , _i : usize) -> Option < & str > { None } fn len (& self) -> usize { 0 } }
    };
}

impl_8!();