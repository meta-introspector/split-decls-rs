macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl Drop for Config { fn drop (& mut self) { unsafe { raw :: git_config_free (self . raw) } } }
    };
}

impl_270!()