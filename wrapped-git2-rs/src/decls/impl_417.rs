macro_rules! deps {
    () => {
        Mempack!();
        Binding!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl < 'odb > Binding for Mempack < 'odb > { type Raw = * mut raw :: git_odb_backend ; unsafe fn from_raw (raw : * mut raw :: git_odb_backend) -> Mempack < 'odb > { Mempack { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_backend { self . raw } }
    };
}

impl_417!();