macro_rules! deps {
    () => {
        Filter!();
        Reference!();
        Derive!();
    };
}

macro_rules! ArgKind {
    () => {
        deps!();
        enum ArgKind { None , Input , Output , Filter , Rustfmt , Reference , Derive , Link , }
    };
}

ArgKind!()