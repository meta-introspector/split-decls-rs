macro_rules! deps {
    () => {
        Repository!();
        Binding!();
    };
}

macro_rules! impl_698 {
    () => {
        deps!();
        impl Binding for Repository { type Raw = * mut raw :: git_repository ; unsafe fn from_raw (ptr : * mut raw :: git_repository) -> Repository { Repository { raw : ptr } } fn raw (& self) -> * mut raw :: git_repository { self . raw } }
    };
}

impl_698!();