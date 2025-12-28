macro_rules! deps {
    () => {
        Derive!();
        Reference!();
        Filter!();
    };
}

macro_rules! ArgKind {
    () => {
        deps!();
        enum ArgKind { None , Input , Output , Filter , Rustfmt , Reference , Derive , Link , }
    };
}

ArgKind!();