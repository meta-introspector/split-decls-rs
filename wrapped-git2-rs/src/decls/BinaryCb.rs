macro_rules! deps {
    () => {
        DiffBinary!();
        DiffDelta!();
    };
}

macro_rules! BinaryCb {
    () => {
        deps!();
        pub type BinaryCb < 'a > = dyn FnMut (DiffDelta < '_ > , DiffBinary < '_ >) -> bool + 'a ;
    };
}

BinaryCb!()