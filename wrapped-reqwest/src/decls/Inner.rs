macro_rules! deps {
    () => {
        BoxError!();
        Kind!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        struct Inner { kind : Kind , source : Option < BoxError > , url : Option < Url > , }
    };
}

Inner!();