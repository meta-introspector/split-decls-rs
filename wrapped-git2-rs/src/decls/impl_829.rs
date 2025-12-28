macro_rules! deps {
    () => {
        Binding!();
        TreeEntry!();
    };
}

macro_rules! impl_829 {
    () => {
        deps!();
        impl < 'a > Binding for TreeEntry < 'a > { type Raw = * mut raw :: git_tree_entry ; unsafe fn from_raw (raw : * mut raw :: git_tree_entry) -> TreeEntry < 'a > { TreeEntry { raw , owned : true , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_tree_entry { self . raw } }
    };
}

impl_829!();