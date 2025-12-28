macro_rules! deps {
    () => {
        TreeBuilder!();
        Binding!();
    };
}

macro_rules! impl_846 {
    () => {
        deps!();
        impl < 'repo > Binding for TreeBuilder < 'repo > { type Raw = * mut raw :: git_treebuilder ; unsafe fn from_raw (raw : * mut raw :: git_treebuilder) -> TreeBuilder < 'repo > { TreeBuilder { raw , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_treebuilder { self . raw } }
    };
}

impl_846!()