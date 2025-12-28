macro_rules! deps {
    () => {
        ResolvePath!();
    };
}

macro_rules! Paths {
    () => {
        deps!();
        pub type Paths = Vec < ResolvePath > ;
    };
}

Paths!();