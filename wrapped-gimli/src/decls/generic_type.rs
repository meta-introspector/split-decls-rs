macro_rules! deps {
    () => {
        ReaderOffset!();
        UnitOffset!();
    };
}

macro_rules! generic_type {
    () => {
        deps!();
        fn generic_type < O : ReaderOffset > () -> UnitOffset < O > { UnitOffset (O :: from_u64 (0) . unwrap ()) }
    };
}

generic_type!();