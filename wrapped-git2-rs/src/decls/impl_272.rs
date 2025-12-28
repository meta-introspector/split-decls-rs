macro_rules! deps {
    () => {
        ConfigEntry!();
        Binding!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl < 'cfg > Binding for ConfigEntry < 'cfg > { type Raw = * mut raw :: git_config_entry ; unsafe fn from_raw (raw : * mut raw :: git_config_entry) -> ConfigEntry < 'cfg > { ConfigEntry { raw , _marker : marker :: PhantomData , owned : true , } } fn raw (& self) -> * mut raw :: git_config_entry { self . raw } }
    };
}

impl_272!();