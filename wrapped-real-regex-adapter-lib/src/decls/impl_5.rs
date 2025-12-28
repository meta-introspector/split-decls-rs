macro_rules! deps {
    () => {
        DummyRegexMatcher!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl DummyRegexMatcher { pub fn new (_re : & str) -> Result < Self > { Ok (DummyRegexMatcher) } }
    };
}

impl_5!();