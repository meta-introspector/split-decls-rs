// Generated macro for FileTypeDef (struct)
macro_rules! Depcrate_typesFileTypeDef {
() => {
// Module: crate::types
// Provides: {"FileTypeDef"}
// Dependencies: {}
# [doc = " A single file type definition."] # [doc = ""] # [doc = " File type definitions can be retrieved in aggregate from a file type"] # [doc = " matcher. File type definitions are also reported when its responsible"] # [doc = " for a match."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct FileTypeDef { name : String , globs : Vec < String > , }
};
}
