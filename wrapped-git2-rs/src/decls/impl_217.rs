macro_rules! deps {
    () => {
        Blob!();
        Binding!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < 'repo > Binding for Blob < 'repo > { type Raw = * mut raw :: git_blob ; unsafe fn from_raw (raw : * mut raw :: git_blob) -> Blob < 'repo > { Blob { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_blob { self . raw } }
    };
}

impl_217!()