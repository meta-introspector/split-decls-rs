macro_rules! deps {
    () => {
        RustcFacts!();
    };
}

macro_rules! PoloniusFacts {
    () => {
        deps!();
        pub type PoloniusFacts = AllFacts < RustcFacts > ;
    };
}

PoloniusFacts!()