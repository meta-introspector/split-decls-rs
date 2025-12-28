macro_rules! deps {
    () => {
        Diff!();
        Binding!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < 'repo > Binding for Diff < 'repo > { type Raw = * mut raw :: git_diff ; unsafe fn from_raw (raw : * mut raw :: git_diff) -> Diff < 'repo > { Diff { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_diff { self . raw } }
    };
}

impl_328!();