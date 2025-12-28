macro_rules! deps {
    () => {
        Binding!();
        Blame!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl < 'repo > Binding for Blame < 'repo > { type Raw = * mut raw :: git_blame ; unsafe fn from_raw (raw : * mut raw :: git_blame) -> Blame < 'repo > { Blame { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_blame { self . raw } }
    };
}

impl_205!();