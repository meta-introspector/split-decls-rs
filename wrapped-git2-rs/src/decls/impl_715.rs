macro_rules! deps {
    () => {
        Revwalk!();
    };
}

macro_rules! impl_715 {
    () => {
        deps!();
        impl < 'repo > Drop for Revwalk < 'repo > { fn drop (& mut self) { unsafe { raw :: git_revwalk_free (self . raw) } } }
    };
}

impl_715!();