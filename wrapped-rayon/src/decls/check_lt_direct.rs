macro_rules! check_lt_direct {
    () => {
        # [test] fn check_lt_direct () { assert ! ((0 .. 1024) . into_par_iter () . lt (1 .. 1024)) ; assert ! (! (1 .. 1024) . into_par_iter () . lt (0 .. 1024)) ; }
    };
}

check_lt_direct!();