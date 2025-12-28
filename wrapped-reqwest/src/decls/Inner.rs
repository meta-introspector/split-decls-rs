macro_rules! deps {
    () => {
        Kind!();
        BoxError!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        struct Inner { kind : Kind , source : Option < BoxError > , url : Option < Url > , }
    };
}

Inner!()