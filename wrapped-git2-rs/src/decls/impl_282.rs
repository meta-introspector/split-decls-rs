macro_rules! deps {
    () => {
        Binding!();
        Cred!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl Binding for Cred { type Raw = * mut raw :: git_cred ; unsafe fn from_raw (raw : * mut raw :: git_cred) -> Cred { Cred { raw } } fn raw (& self) -> * mut raw :: git_cred { self . raw } }
    };
}

impl_282!();