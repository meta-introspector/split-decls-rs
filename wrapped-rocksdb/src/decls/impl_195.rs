macro_rules! deps {
    () => {
        BlockBasedOptions!();
        BlockBasedOptionsMustOutliveDB!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl Default for BlockBasedOptions { fn default () -> Self { let block_opts = unsafe { ffi :: rocksdb_block_based_options_create () } ; assert ! (! block_opts . is_null () , "Could not create RocksDB block based options") ; Self { inner : block_opts , outlive : BlockBasedOptionsMustOutliveDB :: default () , } } }
    };
}

impl_195!();