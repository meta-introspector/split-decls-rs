macro_rules! PerfContext {
    () => {
        # [doc = " Thread local context for gathering performance counter efficiently"] # [doc = " and transparently."] pub struct PerfContext { pub (crate) inner : * mut ffi :: rocksdb_perfcontext_t , }
    };
}

PerfContext!()