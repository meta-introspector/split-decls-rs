macro_rules! deps {
    () => {
        Repository!();
        ThreadSafeRepository!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl From < crate :: Repository > for crate :: ThreadSafeRepository { fn from (r : crate :: Repository) -> Self { crate :: ThreadSafeRepository { refs : r . refs , objects : r . objects . into_inner () . store () , work_tree : r . work_tree , common_dir : r . common_dir , config : r . config , linked_worktree_options : r . options , # [cfg (feature = "index")] index : r . index , # [cfg (feature = "attributes")] modules : r . modules , shallow_commits : r . shallow_commits , } } }
    };
}

impl_334!()