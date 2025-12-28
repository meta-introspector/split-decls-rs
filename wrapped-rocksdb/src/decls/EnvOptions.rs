macro_rules! EnvOptions {
    () => {
        struct EnvOptions { inner : * mut ffi :: rocksdb_envoptions_t , }
    };
}

EnvOptions!()