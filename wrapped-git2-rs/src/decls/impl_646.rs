macro_rules! deps {
    () => {
        Binding!();
        Remote!();
    };
}

macro_rules! impl_646 {
    () => {
        deps!();
        impl < 'repo > Binding for Remote < 'repo > { type Raw = * mut raw :: git_remote ; unsafe fn from_raw (raw : * mut raw :: git_remote) -> Remote < 'repo > { Remote { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_remote { self . raw } }
    };
}

impl_646!()