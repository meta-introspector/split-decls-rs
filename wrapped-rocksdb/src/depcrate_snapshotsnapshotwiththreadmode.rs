// Generated macro for SnapshotWithThreadMode (struct)
macro_rules! Depcrate_snapshotSnapshotWithThreadMode {
() => {
// Module: crate::snapshot
// Provides: {"SnapshotWithThreadMode"}
// Dependencies: {}
# [doc = " A consistent view of the database at the point of creation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rocksdb::{DB, IteratorMode, Options};"] # [doc = ""] # [doc = " let tempdir = tempfile::Builder::new()"] # [doc = "     .prefix(\"_path_for_rocksdb_storage3\")"] # [doc = "     .tempdir()"] # [doc = "     .expect(\"Failed to create temporary path for the _path_for_rocksdb_storage3\");"] # [doc = " let path = tempdir.path();"] # [doc = " {"] # [doc = "     let db = DB::open_default(path).unwrap();"] # [doc = "     let snapshot = db.snapshot(); // Creates a longer-term snapshot of the DB, but closed when goes out of scope"] # [doc = "     let mut iter = snapshot.iterator(IteratorMode::Start); // Make as many iterators as you'd like from one snapshot"] # [doc = " }"] # [doc = " let _ = DB::destroy(&Options::default(), path);"] # [doc = " ```"] # [doc = ""] pub struct SnapshotWithThreadMode < 'a , D : DBAccess > { db : & 'a D , pub (crate) inner : * const ffi :: rocksdb_snapshot_t , }
};
}
