macro_rules! deps {
    () => {
        Describe!();
        Binding!();
    };
}

macro_rules! impl_291 {
    () => {
        deps!();
        impl < 'repo > Binding for Describe < 'repo > { type Raw = * mut raw :: git_describe_result ; unsafe fn from_raw (raw : * mut raw :: git_describe_result) -> Describe < 'repo > { Describe { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_describe_result { self . raw } }
    };
}

impl_291!()