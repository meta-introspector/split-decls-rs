// Generated macro for opendir (function)
macro_rules! Depcrate_fsopendir {
() => {
// Module: crate::fs
// Provides: {"opendir"}
// Dependencies: {}
# [doc = " Open a directory to read the directory entries"] pub (crate) fn opendir (name : & str) -> io :: Result < FileDescriptor > { let obj = FILESYSTEM . get () . ok_or (Errno :: Inval) ? . opendir (name) ? ; insert_object (obj) }
};
}
