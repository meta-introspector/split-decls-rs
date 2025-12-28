macro_rules! deps {
    () => {
        Binding!();
        Pathspec!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl Binding for Pathspec { type Raw = * mut raw :: git_pathspec ; unsafe fn from_raw (raw : * mut raw :: git_pathspec) -> Pathspec { Pathspec { raw } } fn raw (& self) -> * mut raw :: git_pathspec { self . raw } }
    };
}

impl_557!()