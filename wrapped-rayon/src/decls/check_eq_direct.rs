macro_rules! check_eq_direct {
    () => {
        # [test] fn check_eq_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (0 .. 1024) . into_par_iter () ; let result = a . eq (b) ; assert ! (result) ; }
    };
}

check_eq_direct!()