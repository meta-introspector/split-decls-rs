// Generated macro for impl_118 (impl)
macro_rules! Depcrate_filesimpl_118 {
() => {
// Module: crate::files
// Provides: {"impl_118"}
// Dependencies: {}
impl FilePositionWrapper < span :: FileId > { pub fn with_edition (self , db : & dyn ExpandDatabase , edition : span :: Edition) -> FilePosition { FilePositionWrapper { file_id : EditionedFileId :: new (db , self . file_id , edition) , offset : self . offset , } } }
};
}
