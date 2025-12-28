macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! ConflictStyle {
    () => {
        deps!();
        # [doc = " The `merge.conflictStyle` key."] # [cfg (feature = "merge")] pub type ConflictStyle = keys :: Any < validate :: ConflictStyle > ;
    };
}

ConflictStyle!();