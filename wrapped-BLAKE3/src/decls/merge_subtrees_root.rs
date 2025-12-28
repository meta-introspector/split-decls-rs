macro_rules! deps {
    () => {
        ChainingValue!();
        Mode!();
        Hash!();
    };
}

macro_rules! merge_subtrees_root {
    () => {
        deps!();
        # [doc = " Compute a root hash from two child chaining values."] # [doc = ""] # [doc = " See the [module level examples](index.html#examples), particularly the discussion of valid tree"] # [doc = " structures. The left and right child chaining values can come from either"] # [doc = " [`Hasher::finalize_non_root`](HasherExt::finalize_non_root) or [`merge_subtrees_non_root`]."] # [doc = " \"Chaining value\" is the academic term for a non-root or non-final hash."] # [doc = ""] # [doc = " Note that inputs of [`CHUNK_LEN`] or less don't produce any parent nodes and can't be hashed"] # [doc = " using this function. In that case you must get the root hash from [`Hasher::finalize`] (or just"] # [doc = " [`blake3::hash`](crate::hash))."] pub fn merge_subtrees_root (left_child : & ChainingValue , right_child : & ChainingValue , mode : Mode ,) -> crate :: Hash { merge_subtrees_inner (left_child , right_child , mode) . root_hash () }
    };
}

merge_subtrees_root!();