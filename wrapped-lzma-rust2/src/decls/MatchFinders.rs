macro_rules! deps {
    () => {
        Hc4!();
        Bt4!();
    };
}

macro_rules! MatchFinders {
    () => {
        deps!();
        pub (crate) enum MatchFinders { Hc4 (Hc4) , Bt4 (Bt4) , }
    };
}

MatchFinders!();