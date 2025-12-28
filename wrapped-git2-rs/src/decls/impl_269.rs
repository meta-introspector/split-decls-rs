macro_rules! deps {
    () => {
        Config!();
        Binding!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl Binding for Config { type Raw = * mut raw :: git_config ; unsafe fn from_raw (raw : * mut raw :: git_config) -> Config { Config { raw } } fn raw (& self) -> * mut raw :: git_config { self . raw } }
    };
}

impl_269!();