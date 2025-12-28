macro_rules! deps {
    () => {
        Odb!();
        Binding!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl < 'repo > Binding for Odb < 'repo > { type Raw = * mut raw :: git_odb ; unsafe fn from_raw (raw : * mut raw :: git_odb) -> Odb < 'repo > { Odb { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_odb { self . raw } }
    };
}

impl_489!();