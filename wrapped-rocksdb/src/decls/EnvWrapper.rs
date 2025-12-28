macro_rules! EnvWrapper {
    () => {
        pub (crate) struct EnvWrapper { pub (crate) inner : * mut ffi :: rocksdb_env_t , }
    };
}

EnvWrapper!();