// Generated macro for impl_115 (impl)
macro_rules! Depcrate_filesimpl_115 {
() => {
// Module: crate::files
// Provides: {"impl_115"}
// Dependencies: {}
impl FilePosition { # [inline] pub fn into_file_id (self , db : & dyn ExpandDatabase) -> FilePositionWrapper < FileId > { FilePositionWrapper { file_id : self . file_id . file_id (db) , offset : self . offset } } }
};
}
