macro_rules! deps {
    () => {
        MergeFileResult!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl Drop for MergeFileResult { fn drop (& mut self) { unsafe { raw :: git_merge_file_result_free (& mut self . raw) } } }
    };
}

impl_433!();