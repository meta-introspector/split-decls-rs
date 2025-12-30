// Generated macro for impl_126 (impl)
macro_rules! Depcrate_filesimpl_126 {
() => {
// Module: crate::files
// Provides: {"impl_126"}
// Dependencies: {}
impl FileRange { # [inline] pub fn into_file_id (self , db : & dyn ExpandDatabase) -> FileRangeWrapper < FileId > { FileRangeWrapper { file_id : self . file_id . file_id (db) , range : self . range } } # [inline] pub fn file_text (self , db : & dyn ExpandDatabase) -> & triomphe :: Arc < str > { db . file_text (self . file_id . file_id (db)) . text (db) } # [inline] pub fn text (self , db : & dyn ExpandDatabase) -> & str { & self . file_text (db) [self . range] } }
};
}
