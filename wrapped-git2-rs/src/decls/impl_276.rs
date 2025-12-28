macro_rules! deps {
    () => {
        ConfigEntry!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl < 'cfg > Drop for ConfigEntry < 'cfg > { fn drop (& mut self) { if self . owned { unsafe { raw :: git_config_entry_free (self . raw) } } } }
    };
}

impl_276!();