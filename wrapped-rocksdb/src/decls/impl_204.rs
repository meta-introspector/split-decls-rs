macro_rules! deps {
    () => {
        WriteOptions!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl Default for WriteOptions { fn default () -> Self { let write_opts = unsafe { ffi :: rocksdb_writeoptions_create () } ; assert ! (! write_opts . is_null () , "Could not create RocksDB write options") ; Self { inner : write_opts } } }
    };
}

impl_204!();