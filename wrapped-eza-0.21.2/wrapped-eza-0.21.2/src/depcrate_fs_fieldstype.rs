// Generated macro for Type (enum)
macro_rules! Depcrate_fs_fieldsType {
() => {
// Module: crate::fs::fields
// Provides: {"Type"}
// Dependencies: {}
# [doc = " The file’s base type, which gets displayed in the very first column of the"] # [doc = " details output."] # [doc = ""] # [doc = " This type is set entirely by the filesystem, rather than relying on a"] # [doc = " file’s contents. So “link” is a type, but “image” is just a type of"] # [doc = " regular file. (See the `filetype` module for those checks.)"] # [doc = ""] # [doc = " Its ordering is used when sorting by type."] # [derive (PartialEq , Eq , PartialOrd , Ord , Copy , Clone)] pub enum Type { Directory , File , Link , Pipe , Socket , CharDevice , BlockDevice , Special , }
};
}
