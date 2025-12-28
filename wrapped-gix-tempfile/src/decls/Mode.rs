macro_rules! deps {
    () => {
        Closed!();
        Writable!();
    };
}

macro_rules! Mode {
    () => {
        deps!();
        pub (crate) enum Mode { Writable , Closed , }
    };
}

Mode!();