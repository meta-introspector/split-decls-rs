macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl < 'repo > Drop for Commit < 'repo > { fn drop (& mut self) { unsafe { raw :: git_commit_free (self . raw) } } }
    };
}

impl_262!();