macro_rules! deps {
    () => {
        ConfigEntries!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl < 'cfg > Drop for ConfigEntries < 'cfg > { fn drop (& mut self) { unsafe { raw :: git_config_iterator_free (self . raw) } } }
    };
}

impl_275!()