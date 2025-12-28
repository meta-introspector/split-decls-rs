macro_rules! deps {
    () => {
        PrefixSet!();
    };
}

macro_rules! Prefixes {
    () => {
        deps!();
        pub (super) struct Prefixes < 'tcx > { kind : PrefixSet , next : Option < PlaceRef < 'tcx > > , }
    };
}

Prefixes!()