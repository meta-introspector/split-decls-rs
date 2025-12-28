macro_rules! DBInner {
    () => {
        # [doc = " Get underlying `rocksdb_t`."] pub trait DBInner { fn inner (& self) -> * mut ffi :: rocksdb_t ; }
    };
}

DBInner!()