// Generated macro for try_rename_file (function)
macro_rules! Depcrate_utilstry_rename_file {
() => {
// Module: crate::utils
// Provides: {"try_rename_file"}
// Dependencies: {}
# [track_caller] pub fn try_rename_file (old_name : & Path , new_name : & Path) -> bool { match OpenOptions :: new () . create_new (true) . write (true) . open (new_name) { Ok (file) => drop (file) , Err (e) if matches ! (e . kind () , io :: ErrorKind :: AlreadyExists | io :: ErrorKind :: NotFound) => return false , Err (ref e) => panic_action (e , ErrAction :: Create , new_name) , } match fs :: rename (old_name , new_name) { Ok (()) => true , Err (ref e) => { drop (fs :: remove_file (new_name)) ; if matches ! (e . kind () , io :: ErrorKind :: NotFound | io :: ErrorKind :: NotADirectory) { false } else { panic_action (e , ErrAction :: Rename , old_name) ; } } , } }
};
}
