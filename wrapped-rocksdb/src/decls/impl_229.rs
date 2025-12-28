macro_rules! deps {
    () => {
        UniversalCompactOptions!();
        Options!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl Default for UniversalCompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_universal_compaction_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Universal Compaction Options") ; Self { inner : opts } } }
    };
}

impl_229!()