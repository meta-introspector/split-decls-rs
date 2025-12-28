macro_rules! deps {
    () => {
        Tree!();
    };
}

macro_rules! impl_825 {
    () => {
        deps!();
        impl < 'repo > Drop for Tree < 'repo > { fn drop (& mut self) { unsafe { raw :: git_tree_free (self . raw) } } }
    };
}

impl_825!()