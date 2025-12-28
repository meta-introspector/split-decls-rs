macro_rules! CuckooTableOptions {
    () => {
        # [doc = " Configuration of cuckoo-based storage."] pub struct CuckooTableOptions { pub (crate) inner : * mut ffi :: rocksdb_cuckoo_table_options_t , }
    };
}

CuckooTableOptions!();