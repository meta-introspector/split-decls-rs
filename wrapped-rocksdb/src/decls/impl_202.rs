macro_rules! deps {
    () => {
        FlushOptions!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl Default for FlushOptions { fn default () -> Self { let flush_opts = unsafe { ffi :: rocksdb_flushoptions_create () } ; assert ! (! flush_opts . is_null () , "Could not create RocksDB flush options") ; Self { inner : flush_opts } } }
    };
}

impl_202!()