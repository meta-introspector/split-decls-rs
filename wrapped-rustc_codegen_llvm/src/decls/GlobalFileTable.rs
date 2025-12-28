macro_rules! GlobalFileTable {
    () => {
        # [doc = " Maps \"global\" (per-CGU) file ID numbers to their underlying source file paths."] # [derive (Debug)] struct GlobalFileTable { # [doc = " This \"raw\" table doesn't include the working dir, so a file's"] # [doc = " global ID is its index in this set **plus one**."] raw_file_table : FxIndexMap < StableSourceFileId , String > , # [doc = " The file table in encoded form (possibly compressed), which can be"] # [doc = " included directly in this CGU's `__llvm_covmap` record."] filenames_buffer : Vec < u8 > , # [doc = " Truncated hash of the bytes in `filenames_buffer`."] # [doc = ""] # [doc = " The `llvm-cov` tool uses this hash to associate each covfun record with"] # [doc = " its corresponding filenames table, since the final binary will typically"] # [doc = " contain multiple covmap records from different compilation units."] filenames_hash : u64 , }
    };
}

GlobalFileTable!()