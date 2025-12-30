// Generated macro for remove_dir (function)
macro_rules! Depcrate_fsremove_dir {
() => {
// Module: crate::fs
// Provides: {"remove_dir"}
// Dependencies: {}
# [doc = " Removes an empty directory."] pub fn remove_dir (path : & str) -> io :: Result < () > { with_relative_filename (path , | path | { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . rmdir (path) }) }
};
}
