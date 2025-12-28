macro_rules! deps {
    () => {
        TreeNode!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl TreeNode { fn is_leaf_node (& self) -> bool { self . children . is_empty () } }
    };
}

impl_141!()