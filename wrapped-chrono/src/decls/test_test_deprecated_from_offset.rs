macro_rules! deps {
    () => {
        DateTime!();
        Local!();
        FixedOffset!();
    };
}

macro_rules! test_test_deprecated_from_offset {
    () => {
        deps!();
        # [test] # [cfg (feature = "clock")] # [allow (deprecated)] fn test_test_deprecated_from_offset () { let now = Local :: now () ; let naive = now . naive_local () ; let utc = now . naive_utc () ; let offset : FixedOffset = * now . offset () ; assert_eq ! (DateTime ::< Local >:: from_local (naive , offset) , now) ; assert_eq ! (DateTime ::< Local >:: from_utc (utc , offset) , now) ; }
    };
}

test_test_deprecated_from_offset!();