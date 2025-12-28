macro_rules! check_le_equal_direct {
    () => {
        # [test] fn check_le_equal_direct () { assert ! ((0 .. 1024) . into_par_iter () . le ((0 .. 1024) . into_par_iter ())) ; }
    };
}

check_le_equal_direct!();