macro_rules! deps {
    () => {
        SupUnit!();
    };
}

macro_rules! SupUnits {
    () => {
        deps!();
        pub (crate) struct SupUnits < R : gimli :: Reader > { units : Box < [SupUnit < R >] > , }
    };
}

SupUnits!()