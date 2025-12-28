macro_rules! deps {
    () => {
        UnitRange!();
        ResUnit!();
    };
}

macro_rules! ResUnits {
    () => {
        deps!();
        pub (crate) struct ResUnits < R : gimli :: Reader > { ranges : Box < [UnitRange] > , units : Box < [ResUnit < R >] > , }
    };
}

ResUnits!();