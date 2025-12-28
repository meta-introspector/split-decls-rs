macro_rules! deps {
    () => {
        CompareFn!();
    };
}

macro_rules! ComparatorCallback {
    () => {
        deps!();
        pub struct ComparatorCallback { pub name : CString , pub compare_fn : Box < CompareFn > , }
    };
}

ComparatorCallback!();