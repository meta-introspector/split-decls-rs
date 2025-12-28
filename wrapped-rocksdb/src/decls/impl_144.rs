macro_rules! deps {
    () => {
        WriteBufferManagerWrapper!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Drop for WriteBufferManagerWrapper { fn drop (& mut self) { unsafe { ffi :: rocksdb_write_buffer_manager_destroy (self . inner . as_ptr ()) ; } } }
    };
}

impl_144!()