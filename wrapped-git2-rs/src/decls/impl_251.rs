macro_rules! deps {
    () => {
        Binding!();
        Commit!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < 'repo > Binding for Commit < 'repo > { type Raw = * mut raw :: git_commit ; unsafe fn from_raw (raw : * mut raw :: git_commit) -> Commit < 'repo > { Commit { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_commit { self . raw } }
    };
}

impl_251!();