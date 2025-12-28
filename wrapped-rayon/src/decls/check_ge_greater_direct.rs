macro_rules! check_ge_greater_direct {
    () => {
        # [test] fn check_ge_greater_direct () { assert ! ((1 .. 1024) . into_par_iter () . ge ((0 .. 1024) . into_par_iter ())) ; }
    };
}

check_ge_greater_direct!()