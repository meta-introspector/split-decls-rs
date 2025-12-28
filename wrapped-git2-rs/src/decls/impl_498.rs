macro_rules! deps {
    () => {
        Binding!();
        OdbReader!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        impl < 'repo > Binding for OdbReader < 'repo > { type Raw = * mut raw :: git_odb_stream ; unsafe fn from_raw (raw : * mut raw :: git_odb_stream) -> OdbReader < 'repo > { OdbReader { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_stream { self . raw } }
    };
}

impl_498!();