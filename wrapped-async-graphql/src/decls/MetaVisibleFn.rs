macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! MetaVisibleFn {
    () => {
        deps!();
        type MetaVisibleFn = fn (& Context < '_ >) -> bool ;
    };
}

MetaVisibleFn!();