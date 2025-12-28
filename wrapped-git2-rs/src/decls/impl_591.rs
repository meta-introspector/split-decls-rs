macro_rules! deps {
    () => {
        Rebase!();
    };
}

macro_rules! impl_591 {
    () => {
        deps!();
        impl < 'repo > Drop for Rebase < 'repo > { fn drop (& mut self) { unsafe { raw :: git_rebase_free (self . raw) } } }
    };
}

impl_591!()