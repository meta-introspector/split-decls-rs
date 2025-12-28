macro_rules! deps {
    () => {
        Writable!();
        Closed!();
    };
}

macro_rules! Mode {
    () => {
        deps!();
        pub (crate) enum Mode { Writable , Closed , }
    };
}

Mode!()