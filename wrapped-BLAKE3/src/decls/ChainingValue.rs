macro_rules! ChainingValue {
    () => {
        # [doc = " \"Chaining value\" is the academic term for a non-root or non-final hash."] # [doc = ""] # [doc = " Besides just sounding fancy, it turns out there are [security"] # [doc = " reasons](https://jacko.io/tree_hashing.html) to be careful about the difference between"] # [doc = " (root/final) hashes and (non-root/non-final) chaining values."] pub type ChainingValue = [u8 ; OUT_LEN] ;
    };
}

ChainingValue!()