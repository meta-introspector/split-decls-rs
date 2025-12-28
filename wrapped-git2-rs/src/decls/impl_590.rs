macro_rules! deps {
    () => {
        Rebase!();
        Binding!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        impl < 'repo > Binding for Rebase < 'repo > { type Raw = * mut raw :: git_rebase ; unsafe fn from_raw (raw : * mut raw :: git_rebase) -> Rebase < 'repo > { Rebase { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_rebase { self . raw } }
    };
}

impl_590!();