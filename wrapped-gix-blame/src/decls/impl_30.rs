macro_rules! deps {
    () => {
        TreeDiffChange!();
        Change!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < gix_diff :: tree_with_rewrites :: Change > for TreeDiffChange { fn from (value : gix_diff :: tree_with_rewrites :: Change) -> Self { use gix_diff :: tree_with_rewrites :: Change ; match value { Change :: Addition { id , .. } => Self :: Addition { id } , Change :: Deletion { .. } => Self :: Deletion , Change :: Modification { previous_id , id , .. } => Self :: Modification { previous_id , id } , Change :: Rewrite { source_location , source_id , id , .. } => Self :: Rewrite { source_location , source_id , id , } , } } }
    };
}

impl_30!();