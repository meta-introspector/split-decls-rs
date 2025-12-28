macro_rules! deps {
    () => {
        Binding!();
        Index!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        impl Binding for Index { type Raw = * mut raw :: git_index ; unsafe fn from_raw (raw : * mut raw :: git_index) -> Index { Index { raw } } fn raw (& self) -> * mut raw :: git_index { self . raw } }
    };
}

impl_388!();