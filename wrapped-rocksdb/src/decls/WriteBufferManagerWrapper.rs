macro_rules! WriteBufferManagerWrapper {
    () => {
        pub (crate) struct WriteBufferManagerWrapper { pub (crate) inner : NonNull < ffi :: rocksdb_write_buffer_manager_t > , }
    };
}

WriteBufferManagerWrapper!();