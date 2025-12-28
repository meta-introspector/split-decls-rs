macro_rules! deps {
    () => {
        Checkpoint!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl Drop for Checkpoint < '_ > { fn drop (& mut self) { unsafe { ffi :: rocksdb_checkpoint_object_destroy (self . inner) ; } } }
    };
}

impl_37!();