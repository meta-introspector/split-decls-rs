macro_rules! deps {
    () => {
        Binding!();
        Reference!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        impl < 'repo > Binding for Reference < 'repo > { type Raw = * mut raw :: git_reference ; unsafe fn from_raw (raw : * mut raw :: git_reference) -> Reference < 'repo > { Reference { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_reference { self . raw } }
    };
}

impl_609!()