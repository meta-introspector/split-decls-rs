macro_rules! deps {
    () => {
        Diff!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < 'repo > Drop for Diff < 'repo > { fn drop (& mut self) { unsafe { raw :: git_diff_free (self . raw) } } }
    };
}

impl_329!();