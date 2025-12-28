macro_rules! deps {
    () => {
        DefaultStrategy!();
    };
}

macro_rules! macro_136 {
    () => {
        deps!();
        t ! (tests_default , crate :: DefaultStrategy) ;
    };
}

macro_136!()