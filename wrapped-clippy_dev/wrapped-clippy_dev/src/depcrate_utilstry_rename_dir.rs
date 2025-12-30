// Generated macro for try_rename_dir (function)
macro_rules! Depcrate_utilstry_rename_dir {
() => {
// Module: crate::utils
// Provides: {"try_rename_dir"}
// Dependencies: {}
# [track_caller] pub fn try_rename_dir (old_name : & Path , new_name : & Path) -> bool { match fs :: create_dir (new_name) { Ok (()) => { } , Err (e) if matches ! (e . kind () , io :: ErrorKind :: AlreadyExists | io :: ErrorKind :: NotFound) => return false , Err (ref e) => panic_action (e , ErrAction :: Create , new_name) , } # [cfg (windows)] drop (fs :: remove_dir (new_name)) ; match fs :: rename (old_name , new_name) { Ok (()) => true , Err (ref e) => { # [cfg (not (windows))] drop (fs :: remove_dir (new_name)) ; if matches ! (e . kind () , io :: ErrorKind :: NotFound | io :: ErrorKind :: NotADirectory) { false } else { panic_action (e , ErrAction :: Rename , old_name) ; } } , } }
};
}
