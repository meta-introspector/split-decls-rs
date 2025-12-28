macro_rules! deps {
    () => {
        IntoFallible!();
    };
}

macro_rules! wrap_std_iter_into_fallible {
    () => {
        deps!();
        # [test] fn wrap_std_iter_into_fallible () { let it = IntoFallible :: from (vec ! [0 , 1 , 2 , 3] . into_iter ()) ; assert_eq ! (it . collect ::< Vec < _ >> () . unwrap () , vec ! [0 , 1 , 2 , 3]) ; }
    };
}

wrap_std_iter_into_fallible!();