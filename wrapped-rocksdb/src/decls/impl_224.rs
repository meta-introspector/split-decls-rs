macro_rules! deps {
    () => {
        Options!();
        FifoCompactOptions!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl Default for FifoCompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_fifo_compaction_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Fifo Compaction Options") ; Self { inner : opts } } }
    };
}

impl_224!()