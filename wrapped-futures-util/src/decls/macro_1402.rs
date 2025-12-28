macro_rules! deps {
    () => {
        IntoFn!();
    };
}

macro_rules! macro_1402 {
    () => {
        deps!();
        trivial_fn_impls ! (into_fn < T > IntoFn < T > = "Into::into") ;
    };
}

macro_1402!();