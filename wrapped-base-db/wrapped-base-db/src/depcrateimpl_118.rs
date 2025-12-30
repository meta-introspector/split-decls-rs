// Generated macro for impl_118 (impl)
macro_rules! Depcrateimpl_118 {
() => {
// Module: crate
// Provides: {"impl_118"}
// Dependencies: {}
impl EditionedFileId { # [inline] pub fn new (db : & dyn salsa :: Database , file_id : FileId , edition : Edition) -> Self { EditionedFileId :: from_span (db , span :: EditionedFileId :: new (file_id , edition)) } # [inline] pub fn current_edition (db : & dyn salsa :: Database , file_id : FileId) -> Self { EditionedFileId :: new (db , file_id , Edition :: CURRENT) } # [inline] pub fn file_id (self , db : & dyn salsa :: Database) -> vfs :: FileId { let id = self . editioned_file_id (db) ; id . file_id () } # [inline] pub fn unpack (self , db : & dyn salsa :: Database) -> (vfs :: FileId , span :: Edition) { let id = self . editioned_file_id (db) ; (id . file_id () , id . edition ()) } # [inline] pub fn edition (self , db : & dyn SourceDatabase) -> Edition { self . editioned_file_id (db) . edition () } }
};
}
