// Generated macro for impl_120 (impl)
macro_rules! Depcrate_filesimpl_120 {
() => {
// Module: crate::files
// Provides: {"impl_120"}
// Dependencies: {}
impl < T > InFileWrapper < span :: FileId , T > { pub fn with_edition (self , db : & dyn ExpandDatabase , edition : span :: Edition) -> InRealFile < T > { InRealFile { file_id : EditionedFileId :: new (db , self . file_id , edition) , value : self . value } } }
};
}
