macro_rules! deps {
    () => {
        OdbWriter!();
        Binding!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl < 'repo > Binding for OdbWriter < 'repo > { type Raw = * mut raw :: git_odb_stream ; unsafe fn from_raw (raw : * mut raw :: git_odb_stream) -> OdbWriter < 'repo > { OdbWriter { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_stream { self . raw } }
    };
}

impl_504!()