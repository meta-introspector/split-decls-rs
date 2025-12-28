macro_rules! deps {
    () => {
        TreeDiffChange!();
        Change!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < gix_diff :: tree :: recorder :: Change > for TreeDiffChange { fn from (value : gix_diff :: tree :: recorder :: Change) -> Self { use gix_diff :: tree :: recorder :: Change ; match value { Change :: Addition { oid , .. } => Self :: Addition { id : oid } , Change :: Deletion { .. } => Self :: Deletion , Change :: Modification { previous_oid , oid , .. } => Self :: Modification { previous_id : previous_oid , id : oid , } , } } }
    };
}

impl_29!()