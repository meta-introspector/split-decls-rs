macro_rules! deps {
    () => {
        CompareFn!();
        CompareTsFn!();
        CompareWithoutTsFn!();
    };
}

macro_rules! ComparatorWithTsCallback {
    () => {
        deps!();
        pub struct ComparatorWithTsCallback { pub name : CString , pub compare_fn : Box < CompareFn > , pub compare_ts_fn : Box < CompareTsFn > , pub compare_without_ts_fn : Box < CompareWithoutTsFn > , }
    };
}

ComparatorWithTsCallback!()