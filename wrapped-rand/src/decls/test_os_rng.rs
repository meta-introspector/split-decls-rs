macro_rules! deps {
    () => {
        OsRng!();
    };
}

macro_rules! test_os_rng {
    () => {
        deps!();
        # [test] fn test_os_rng () { let x = OsRng . try_next_u64 () . unwrap () ; let y = OsRng . try_next_u64 () . unwrap () ; assert ! (x != 0) ; assert ! (x != y) ; }
    };
}

test_os_rng!()