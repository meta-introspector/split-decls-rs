macro_rules! deps {
    () => {
        OdbObject!();
        Binding!();
    };
}

macro_rules! impl_493 {
    () => {
        deps!();
        impl < 'a > Binding for OdbObject < 'a > { type Raw = * mut raw :: git_odb_object ; unsafe fn from_raw (raw : * mut raw :: git_odb_object) -> OdbObject < 'a > { OdbObject { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb_object { self . raw } }
    };
}

impl_493!()