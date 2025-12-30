// Generated macro for impl_119 (impl)
macro_rules! Depcrate_filesimpl_119 {
() => {
// Module: crate::files
// Provides: {"impl_119"}
// Dependencies: {}
impl FileRangeWrapper < span :: FileId > { pub fn with_edition (self , db : & dyn ExpandDatabase , edition : span :: Edition) -> FileRange { FileRangeWrapper { file_id : EditionedFileId :: new (db , self . file_id , edition) , range : self . range , } } }
};
}
