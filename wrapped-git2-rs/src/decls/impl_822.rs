macro_rules! deps {
    () => {
        Binding!();
        Tree!();
    };
}

macro_rules! impl_822 {
    () => {
        deps!();
        impl < 'repo > Binding for Tree < 'repo > { type Raw = * mut raw :: git_tree ; unsafe fn from_raw (raw : * mut raw :: git_tree) -> Tree < 'repo > { Tree { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_tree { self . raw } }
    };
}

impl_822!();