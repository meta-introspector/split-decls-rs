macro_rules! deps {
    () => {
        DefaultStrategy!();
    };
}

macro_rules! macro_162 {
    () => {
        deps!();
        t ! (tests_default , DefaultStrategy) ;
    };
}

macro_162!();