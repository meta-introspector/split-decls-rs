macro_rules! deps {
    () => {
        MergeResultFn!();
    };
}

macro_rules! macro_1366 {
    () => {
        deps!();
        trivial_fn_impls ! (merge_result_fn <> MergeResultFn = "merge_result") ;
    };
}

macro_1366!();