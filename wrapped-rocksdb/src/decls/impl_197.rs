macro_rules! deps {
    () => {
        CuckooTableOptions!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl Default for CuckooTableOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_cuckoo_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB cuckoo options") ; Self { inner : opts } } }
    };
}

impl_197!();