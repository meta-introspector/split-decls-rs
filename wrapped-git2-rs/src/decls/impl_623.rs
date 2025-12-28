macro_rules! deps {
    () => {
        Reflog!();
    };
}

macro_rules! impl_623 {
    () => {
        deps!();
        impl Drop for Reflog { fn drop (& mut self) { unsafe { raw :: git_reflog_free (self . raw) } } }
    };
}

impl_623!()