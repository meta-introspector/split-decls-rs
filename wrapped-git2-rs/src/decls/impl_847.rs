macro_rules! deps {
    () => {
        TreeBuilder!();
    };
}

macro_rules! impl_847 {
    () => {
        deps!();
        impl < 'repo > Drop for TreeBuilder < 'repo > { fn drop (& mut self) { unsafe { raw :: git_treebuilder_free (self . raw) } } }
    };
}

impl_847!();