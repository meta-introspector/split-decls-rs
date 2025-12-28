macro_rules! deps {
    () => {
        TreeEntry!();
    };
}

macro_rules! impl_835 {
    () => {
        deps!();
        impl < 'a > Drop for TreeEntry < 'a > { fn drop (& mut self) { if self . owned { unsafe { raw :: git_tree_entry_free (self . raw) } } } }
    };
}

impl_835!()