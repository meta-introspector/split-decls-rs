macro_rules! deps {
    () => {
        RestoreOptions!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Default for RestoreOptions { fn default () -> Self { unsafe { let opts = ffi :: rocksdb_restore_options_create () ; assert ! (! opts . is_null () , "Could not create RocksDB restore options") ; Self { inner : opts } } } }
    };
}

impl_28!()