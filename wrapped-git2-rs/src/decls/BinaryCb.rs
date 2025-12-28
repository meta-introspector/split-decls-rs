macro_rules! deps {
    () => {
        DiffDelta!();
        DiffBinary!();
    };
}

macro_rules! BinaryCb {
    () => {
        deps!();
        pub type BinaryCb < 'a > = dyn FnMut (DiffDelta < '_ > , DiffBinary < '_ >) -> bool + 'a ;
    };
}

BinaryCb!();