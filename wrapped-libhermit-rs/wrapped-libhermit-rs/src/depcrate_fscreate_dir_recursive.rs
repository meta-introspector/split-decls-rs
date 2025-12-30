// Generated macro for create_dir_recursive (function)
macro_rules! Depcrate_fscreate_dir_recursive {
() => {
// Module: crate::fs
// Provides: {"create_dir_recursive"}
// Dependencies: {}
# [doc = " Creates a directory and creates all missing parent directories as well."] fn create_dir_recursive (path : & str , mode : AccessPermission) -> io :: Result < () > { trace ! ("create_dir_recursive: {path}") ; create_dir (path , mode) . or_else (| errno | { if errno != Errno :: Badf { return Err (errno) ; } let (parent_path , _file_name) = path . rsplit_once ('/') . unwrap () ; create_dir_recursive (parent_path , mode) ? ; create_dir (path , mode) }) }
};
}
