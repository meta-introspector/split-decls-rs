macro_rules! deps {
    () => {
        IID!();
    };
}

macro_rules! REFIID {
    () => {
        deps!();
        pub type REFIID = * const IID ;
    };
}

REFIID!();