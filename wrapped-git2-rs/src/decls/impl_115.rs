macro_rules! deps {
    () => {
        Cert!();
        Binding!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < 'a > Binding for Cert < 'a > { type Raw = * mut raw :: git_cert ; unsafe fn from_raw (raw : * mut raw :: git_cert) -> Cert < 'a > { Cert { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_cert { self . raw } }
    };
}

impl_115!()