macro_rules! deps {
    () => {
        RustcFacts!();
    };
}

macro_rules! PoloniusOutput {
    () => {
        deps!();
        pub type PoloniusOutput = Output < RustcFacts > ;
    };
}

PoloniusOutput!();