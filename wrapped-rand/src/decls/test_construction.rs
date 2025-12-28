macro_rules! deps {
    () => {
        OsRng!();
    };
}

macro_rules! test_construction {
    () => {
        deps!();
        # [test] fn test_construction () { assert ! (OsRng . try_next_u64 () . unwrap () != 0) ; }
    };
}

test_construction!();