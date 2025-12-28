macro_rules! deps {
    () => {
        Options!();
        WaitForCompactOptions!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl Default for WaitForCompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_wait_for_compact_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Wait For Compact Options") ; Self { inner : opts } } }
    };
}

impl_238!()