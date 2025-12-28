macro_rules! impl_10 {
    () => {
        impl MethodCallAttributes { pub const HASTHIS : Self = Self (0x20) ; pub const VARARG : Self = Self (0x05) ; }
    };
}

impl_10!();