// Generated macro for FlushOptions (struct)
macro_rules! Depcrate_db_optionsFlushOptions {
() => {
// Module: crate::db_options
// Provides: {"FlushOptions"}
// Dependencies: {}
# [doc = " Optionally wait for the memtable flush to be performed."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Manually flushing the memtable:"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{DB, Options, FlushOptions};"] # [doc = ""] # [doc = " let tempdir = tempfile::Builder::new()"] # [doc = "     .prefix(\"_path_for_rocksdb_storageY2\")"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary path for the _path_for_rocksdb_storageY2\");"] # [doc = " let path = tempdir.path();"] # [doc = " {"] # [doc = "     let db = DB::open_default(path).unwrap();"] # [doc = ""] # [doc = "     let mut flush_options = FlushOptions::default();"] # [doc = "     flush_options.set_wait(true);"] # [doc = ""] # [doc = "     db.flush_opt(&flush_options);"] # [doc = " }"] # [doc = " let _ = DB::destroy(&Options::default(), path);"] # [doc = " ```"] pub struct FlushOptions { pub (crate) inner : * mut ffi :: rocksdb_flushoptions_t , }
};
}
