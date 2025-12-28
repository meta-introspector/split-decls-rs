macro_rules! deps {
    () => {
        OidArray!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl Drop for OidArray { fn drop (& mut self) { unsafe { raw :: git_oidarray_free (& mut self . raw) } } }
    };
}

impl_121!()