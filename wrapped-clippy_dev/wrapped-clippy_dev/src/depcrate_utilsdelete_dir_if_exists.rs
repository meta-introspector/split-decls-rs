// Generated macro for delete_dir_if_exists (function)
macro_rules! Depcrate_utilsdelete_dir_if_exists {
() => {
// Module: crate::utils
// Provides: {"delete_dir_if_exists"}
// Dependencies: {}
# [track_caller] pub fn delete_dir_if_exists (path : & Path) { match fs :: remove_dir_all (path) { Ok (()) => { } , Err (e) if matches ! (e . kind () , io :: ErrorKind :: NotFound | io :: ErrorKind :: NotADirectory) => { } , Err (ref e) => panic_action (e , ErrAction :: Delete , path) , } }
};
}
