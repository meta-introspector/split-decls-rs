macro_rules! deps {
    () => {
        Binding!();
        Submodule!();
    };
}

macro_rules! impl_765 {
    () => {
        deps!();
        impl < 'repo > Binding for Submodule < 'repo > { type Raw = * mut raw :: git_submodule ; unsafe fn from_raw (raw : * mut raw :: git_submodule) -> Submodule < 'repo > { Submodule { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_submodule { self . raw } }
    };
}

impl_765!()