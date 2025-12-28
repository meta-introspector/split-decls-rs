macro_rules! deps {
    () => {
        Revwalk!();
        Binding!();
    };
}

macro_rules! impl_714 {
    () => {
        deps!();
        impl < 'repo > Binding for Revwalk < 'repo > { type Raw = * mut raw :: git_revwalk ; unsafe fn from_raw (raw : * mut raw :: git_revwalk) -> Revwalk < 'repo > { Revwalk { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_revwalk { self . raw } }
    };
}

impl_714!();