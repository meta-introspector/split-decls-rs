macro_rules! deps {
    () => {
        Binding!();
        MergeFileResult!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl Binding for MergeFileResult { type Raw = raw :: git_merge_file_result ; unsafe fn from_raw (raw : raw :: git_merge_file_result) -> MergeFileResult { MergeFileResult { raw } } fn raw (& self) -> raw :: git_merge_file_result { unimplemented ! () } }
    };
}

impl_432!()