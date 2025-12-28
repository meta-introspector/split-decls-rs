macro_rules! deps {
    () => {
        Mode!();
        ChainingValue!();
    };
}

macro_rules! merge_subtrees_non_root {
    () => {
        deps!();
        # [doc = " Compute a non-root parent node chaining value from two child chaining values."] # [doc = ""] # [doc = " See the [module level examples](index.html#examples), particularly the discussion of valid tree"] # [doc = " structures. The left and right child chaining values can come from either"] # [doc = " [`Hasher::finalize_non_root`](HasherExt::finalize_non_root) or other calls to"] # [doc = " `merge_subtrees_non_root`. \"Chaining value\" is the academic term for a non-root or non-final"] # [doc = " hash."] pub fn merge_subtrees_non_root (left_child : & ChainingValue , right_child : & ChainingValue , mode : Mode ,) -> ChainingValue { merge_subtrees_inner (left_child , right_child , mode) . chaining_value () }
    };
}

merge_subtrees_non_root!();