macro_rules! deps {
    () => {
        Reflog!();
        Binding!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl Binding for Reflog { type Raw = * mut raw :: git_reflog ; unsafe fn from_raw (raw : * mut raw :: git_reflog) -> Reflog { Reflog { raw } } fn raw (& self) -> * mut raw :: git_reflog { self . raw } }
    };
}

impl_622!();