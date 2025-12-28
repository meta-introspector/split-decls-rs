macro_rules! deps {
    () => {
        Binding!();
        Tag!();
    };
}

macro_rules! impl_775 {
    () => {
        deps!();
        impl < 'repo > Binding for Tag < 'repo > { type Raw = * mut raw :: git_tag ; unsafe fn from_raw (raw : * mut raw :: git_tag) -> Tag < 'repo > { Tag { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_tag { self . raw } }
    };
}

impl_775!();