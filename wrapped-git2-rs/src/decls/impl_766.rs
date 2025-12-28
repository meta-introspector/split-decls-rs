macro_rules! deps {
    () => {
        Submodule!();
    };
}

macro_rules! impl_766 {
    () => {
        deps!();
        impl < 'repo > Drop for Submodule < 'repo > { fn drop (& mut self) { unsafe { raw :: git_submodule_free (self . raw) } } }
    };
}

impl_766!();