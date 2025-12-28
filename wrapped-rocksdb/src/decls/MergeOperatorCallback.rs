macro_rules! deps {
    () => {
        MergeFn!();
    };
}

macro_rules! MergeOperatorCallback {
    () => {
        deps!();
        pub struct MergeOperatorCallback < F : MergeFn , PF : MergeFn > { pub name : CString , pub full_merge_fn : F , pub partial_merge_fn : PF , }
    };
}

MergeOperatorCallback!();