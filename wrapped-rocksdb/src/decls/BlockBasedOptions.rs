macro_rules! deps {
    () => {
        BlockBasedOptionsMustOutliveDB!();
    };
}

macro_rules! BlockBasedOptions {
    () => {
        deps!();
        # [doc = " For configuring block-based file storage."] pub struct BlockBasedOptions { pub (crate) inner : * mut ffi :: rocksdb_block_based_table_options_t , outlive : BlockBasedOptionsMustOutliveDB , }
    };
}

BlockBasedOptions!()