macro_rules! deps {
    () => {
        ConfigEntries!();
        Binding!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl < 'cfg > Binding for ConfigEntries < 'cfg > { type Raw = * mut raw :: git_config_iterator ; unsafe fn from_raw (raw : * mut raw :: git_config_iterator) -> ConfigEntries < 'cfg > { ConfigEntries { raw , current : None , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_config_iterator { self . raw } }
    };
}

impl_273!();