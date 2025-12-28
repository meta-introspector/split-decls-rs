macro_rules! deps {
    () => {
        OidArray!();
        Binding!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl Binding for OidArray { type Raw = raw :: git_oidarray ; unsafe fn from_raw (raw : raw :: git_oidarray) -> OidArray { OidArray { raw } } fn raw (& self) -> raw :: git_oidarray { self . raw } }
    };
}

impl_119!()