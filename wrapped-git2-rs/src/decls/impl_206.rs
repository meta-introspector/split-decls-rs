macro_rules! deps {
    () => {
        Blame!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'repo > Drop for Blame < 'repo > { fn drop (& mut self) { unsafe { raw :: git_blame_free (self . raw) } } }
    };
}

impl_206!();