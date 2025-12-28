macro_rules! check_ge_equal_direct {
    () => {
        # [test] fn check_ge_equal_direct () { assert ! ((0 .. 1024) . into_par_iter () . ge ((0 .. 1024) . into_par_iter ())) ; }
    };
}

check_ge_equal_direct!();