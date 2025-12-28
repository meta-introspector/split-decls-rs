macro_rules! deps {
    () => {
        CompactionFilterFn!();
    };
}

macro_rules! CompactionFilterCallback {
    () => {
        deps!();
        pub struct CompactionFilterCallback < F > where F : CompactionFilterFn , { pub name : CString , pub filter_fn : F , }
    };
}

CompactionFilterCallback!();