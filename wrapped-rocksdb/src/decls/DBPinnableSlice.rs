macro_rules! deps {
    () => {
        DB!();
    };
}

macro_rules! DBPinnableSlice {
    () => {
        deps!();
        # [doc = " Wrapper around RocksDB PinnableSlice struct."] # [doc = ""] # [doc = " With a pinnable slice, we can directly leverage in-memory data within"] # [doc = " RocksDB to avoid unnecessary memory copies. The struct here wraps the"] # [doc = " returned raw pointer and ensures proper finalization work."] pub struct DBPinnableSlice < 'a > { ptr : * mut ffi :: rocksdb_pinnableslice_t , db : PhantomData < & 'a DB > , }
    };
}

DBPinnableSlice!()