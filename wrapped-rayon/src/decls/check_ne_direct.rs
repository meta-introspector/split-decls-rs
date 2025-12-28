macro_rules! check_ne_direct {
    () => {
        # [test] fn check_ne_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (1 .. 1024) . into_par_iter () ; let result = a . ne (b) ; assert ! (result) ; }
    };
}

check_ne_direct!()