// Generated macro for file_attributes (function)
macro_rules! Depcrate_fsfile_attributes {
() => {
// Module: crate::fs
// Provides: {"file_attributes"}
// Dependencies: {}
pub fn file_attributes (path : & str) -> io :: Result < FileAttr > { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . lstat (path) }
};
}
