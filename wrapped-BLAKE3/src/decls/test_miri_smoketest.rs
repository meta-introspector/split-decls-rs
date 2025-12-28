macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_miri_smoketest {
    () => {
        deps!();
        # [test] fn test_miri_smoketest () { let mut hasher = crate :: Hasher :: new_derive_key ("Miri smoketest") ; hasher . update (b"foo") ; # [cfg (feature = "std")] hasher . update_reader (& b"bar" [..]) . unwrap () ; assert_eq ! (hasher . finalize () , hasher . finalize ()) ; let mut reader = hasher . finalize_xof () ; reader . set_position (999999) ; reader . fill (& mut [0]) ; }
    };
}

test_miri_smoketest!()