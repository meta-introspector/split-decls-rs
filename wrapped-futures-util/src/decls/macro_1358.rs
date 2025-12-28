macro_rules! deps {
    () => {
        OkFn!();
    };
}

macro_rules! macro_1358 {
    () => {
        deps!();
        trivial_fn_impls ! (ok_fn < T > OkFn < T > = "Ok") ;
    };
}

macro_1358!();