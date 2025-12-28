macro_rules! deps {
    () => {
        Binding!();
        DiffStats!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl Binding for DiffStats { type Raw = * mut raw :: git_diff_stats ; unsafe fn from_raw (raw : * mut raw :: git_diff_stats) -> DiffStats { DiffStats { raw } } fn raw (& self) -> * mut raw :: git_diff_stats { self . raw } }
    };
}

impl_351!()