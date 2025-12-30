// Generated macro for read_lstat (function)
macro_rules! Depcrate_fsread_lstat {
() => {
// Module: crate::fs
// Provides: {"read_lstat"}
// Dependencies: {}
pub fn read_lstat (name : & str) -> io :: Result < FileAttr > { with_relative_filename (name , | name | { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . lstat (name) }) }
};
}
