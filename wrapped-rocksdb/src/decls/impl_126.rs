macro_rules! deps {
    () => {
        DBAccess!();
        DBRawIteratorWithThreadMode!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < D : DBAccess > Drop for DBRawIteratorWithThreadMode < '_ , D > { fn drop (& mut self) { unsafe { ffi :: rocksdb_iter_destroy (self . inner . as_ptr ()) ; } } }
    };
}

impl_126!();