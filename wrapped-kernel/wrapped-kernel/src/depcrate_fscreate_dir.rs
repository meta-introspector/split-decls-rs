// Generated macro for create_dir (function)
macro_rules! Depcrate_fscreate_dir {
() => {
// Module: crate::fs
// Provides: {"create_dir"}
// Dependencies: {}
# [doc = " Creates a new, empty directory at the provided path"] pub fn create_dir (path : & str , mode : AccessPermission) -> io :: Result < () > { let mask = * UMASK . lock () ; with_relative_filename (path , | path | { FILESYSTEM . get () . ok_or (Errno :: Inval) ? . mkdir (path , mode . bitand (mask)) }) }
};
}
