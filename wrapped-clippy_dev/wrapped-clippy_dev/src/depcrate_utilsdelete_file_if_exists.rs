// Generated macro for delete_file_if_exists (function)
macro_rules! Depcrate_utilsdelete_file_if_exists {
() => {
// Module: crate::utils
// Provides: {"delete_file_if_exists"}
// Dependencies: {}
# [track_caller] pub fn delete_file_if_exists (path : & Path) -> bool { match fs :: remove_file (path) { Ok (()) => true , Err (e) if matches ! (e . kind () , io :: ErrorKind :: NotFound | io :: ErrorKind :: IsADirectory) => false , Err (ref e) => panic_action (e , ErrAction :: Delete , path) , } }
};
}
