macro_rules! deps {
    () => {
        CompactOptions!();
        Options!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl Default for CompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_compactoptions_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Compact Options") ; Self { inner : opts , full_history_ts_low : None , } } }
    };
}

impl_234!()