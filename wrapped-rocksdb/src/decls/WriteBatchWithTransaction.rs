macro_rules! deps {
    () => {
        Transaction!();
        Options!();
    };
}

macro_rules! WriteBatchWithTransaction {
    () => {
        deps!();
        # [doc = " An atomic batch of write operations."] # [doc = ""] # [doc = " [`delete_range`](#method.delete_range) is not supported in [`Transaction`]."] # [doc = ""] # [doc = " Making an atomic commit of several writes:"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{DB, Options, WriteBatchWithTransaction};"] # [doc = ""] # [doc = " let tempdir = tempfile::Builder::new()"] # [doc = "     .prefix(\"_path_for_rocksdb_storage1\")"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary path for the _path_for_rocksdb_storage1\");"] # [doc = " let path = tempdir.path();"] # [doc = " {"] # [doc = "     let db = DB::open_default(path).unwrap();"] # [doc = "     let mut batch = WriteBatchWithTransaction::<false>::default();"] # [doc = "     batch.put(b\"my key\", b\"my value\");"] # [doc = "     batch.put(b\"key2\", b\"value2\");"] # [doc = "     batch.put(b\"key3\", b\"value3\");"] # [doc = ""] # [doc = "     // delete_range is supported when use without transaction"] # [doc = "     batch.delete_range(b\"key2\", b\"key3\");"] # [doc = ""] # [doc = "     db.write(batch); // Atomically commits the batch"] # [doc = " }"] # [doc = " let _ = DB::destroy(&Options::default(), path);"] # [doc = " ```"] # [doc = ""] # [doc = " [`Transaction`]: crate::Transaction"] pub struct WriteBatchWithTransaction < const TRANSACTION : bool > { pub (crate) inner : * mut ffi :: rocksdb_writebatch_t , }
    };
}

WriteBatchWithTransaction!()