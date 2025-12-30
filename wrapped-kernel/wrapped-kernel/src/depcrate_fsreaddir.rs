// Generated macro for readdir (function)
macro_rules! Depcrate_fsreaddir {
() => {
// Module: crate::fs
// Provides: {"readdir"}
// Dependencies: {}
# [doc = " Returns an vector with all the entries within a directory."] pub fn readdir (name : & str) -> io :: Result < Vec < DirectoryEntry > > { debug ! ("Read directory {name}") ; with_relative_filename (name , | name | { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . readdir (name) }) }
};
}
