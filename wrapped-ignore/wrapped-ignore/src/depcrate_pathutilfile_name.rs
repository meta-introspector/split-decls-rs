// Generated macro for file_name (function)
macro_rules! Depcrate_pathutilfile_name {
() => {
// Module: crate::pathutil
// Provides: {"file_name"}
// Dependencies: {}
# [doc = " The final component of the path, if it is a normal file."] # [doc = ""] # [doc = " If the path terminates in ., .., or consists solely of a root of prefix,"] # [doc = " file_name will return None."] # [cfg (not (unix))] pub (crate) fn file_name < 'a , P : AsRef < Path > + ? Sized > (path : & 'a P ,) -> Option < & 'a OsStr > { path . as_ref () . file_name () }
};
}
