macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_1025 {
    () => {
        deps!();
        impl From < index_worktree :: Item > for Item { fn from (value : index_worktree :: Item) -> Self { Item :: IndexWorktree (value) } }
    };
}

impl_1025!();