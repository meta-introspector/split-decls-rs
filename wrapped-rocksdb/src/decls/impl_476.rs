macro_rules! deps {
    () => {
        WriteBatchWithTransaction!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl < const TRANSACTION : bool > Drop for WriteBatchWithTransaction < TRANSACTION > { fn drop (& mut self) { unsafe { ffi :: rocksdb_writebatch_destroy (self . inner) ; } } }
    };
}

impl_476!()