macro_rules! deps {
    () => {
        UnitOffset!();
        ReaderOffset!();
    };
}

macro_rules! generic_type {
    () => {
        deps!();
        fn generic_type < O : ReaderOffset > () -> UnitOffset < O > { UnitOffset (O :: from_u64 (0) . unwrap ()) }
    };
}

generic_type!()