// Generated macro for FileEntry (struct)
macro_rules! Depcrate_read_lineFileEntry {
() => {
// Module: crate::read::line
// Provides: {"FileEntry"}
// Dependencies: {}
# [doc = " An entry in the `LineProgramHeader`'s `file_names` set."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct FileEntry < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { path_name : AttributeValue < R , Offset > , directory_index : u64 , timestamp : u64 , size : u64 , md5 : [u8 ; 16] , source : Option < AttributeValue < R , Offset > > , }
};
}
