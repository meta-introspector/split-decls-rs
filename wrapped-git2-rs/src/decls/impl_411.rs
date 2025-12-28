macro_rules! deps {
    () => {
        Mailmap!();
        Binding!();
    };
}

macro_rules! impl_411 {
    () => {
        deps!();
        impl Binding for Mailmap { type Raw = * mut raw :: git_mailmap ; unsafe fn from_raw (ptr : * mut raw :: git_mailmap) -> Mailmap { Mailmap { raw : ptr } } fn raw (& self) -> * mut raw :: git_mailmap { self . raw } }
    };
}

impl_411!();