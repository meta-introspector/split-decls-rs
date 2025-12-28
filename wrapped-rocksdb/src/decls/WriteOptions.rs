macro_rules! deps {
    () => {
        WriteBatch!();
        Options!();
    };
}

macro_rules! WriteOptions {
    () => {
        deps!();
        # [doc = " Optionally disable WAL or sync for this write."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Making an unsafe write of a batch:"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{DB, Options, WriteBatch, WriteOptions};"] # [doc = ""] # [doc = " let tempdir = tempfile::Builder::new()"] # [doc = "     .prefix(\"_path_for_rocksdb_storageY1\")"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary path for the _path_for_rocksdb_storageY1\");"] # [doc = " let path = tempdir.path();"] # [doc = " {"] # [doc = "     let db = DB::open_default(path).unwrap();"] # [doc = "     let mut batch = WriteBatch::default();"] # [doc = "     batch.put(b\"my key\", b\"my value\");"] # [doc = "     batch.put(b\"key2\", b\"value2\");"] # [doc = "     batch.put(b\"key3\", b\"value3\");"] # [doc = ""] # [doc = "     let mut write_options = WriteOptions::default();"] # [doc = "     write_options.set_sync(false);"] # [doc = "     write_options.disable_wal(true);"] # [doc = ""] # [doc = "     db.write_opt(batch, &write_options);"] # [doc = " }"] # [doc = " let _ = DB::destroy(&Options::default(), path);"] # [doc = " ```"] pub struct WriteOptions { pub (crate) inner : * mut ffi :: rocksdb_writeoptions_t , }
    };
}

WriteOptions!()