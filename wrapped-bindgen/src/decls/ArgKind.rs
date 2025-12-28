macro_rules! ArgKind {
    () => {
        enum ArgKind { None , Input , Output , Filter , Rustfmt , Reference , Derive , Link , }
    };
}

ArgKind!()