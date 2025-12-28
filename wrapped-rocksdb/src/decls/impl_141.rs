macro_rules! deps {
    () => {
        DBWALIterator!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl Drop for DBWALIterator { fn drop (& mut self) { unsafe { ffi :: rocksdb_wal_iter_destroy (self . inner) ; } } }
    };
}

impl_141!();