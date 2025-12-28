macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl Clone for Options { fn clone (& self) -> Self { let inner = unsafe { ffi :: rocksdb_options_create_copy (self . inner) } ; assert ! (! inner . is_null () , "Could not copy RocksDB options") ; Self { inner , outlive : self . outlive . clone () , } } }
    };
}

impl_186!();