macro_rules! deps {
    () => {
        Binding!();
        DiffDelta!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl < 'a > Binding for DiffDelta < 'a > { type Raw = * mut raw :: git_diff_delta ; unsafe fn from_raw (raw : * mut raw :: git_diff_delta) -> DiffDelta < 'a > { DiffDelta { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_diff_delta { self . raw } }
    };
}

impl_331!()