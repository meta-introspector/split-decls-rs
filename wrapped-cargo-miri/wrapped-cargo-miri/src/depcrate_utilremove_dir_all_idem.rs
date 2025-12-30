// Generated macro for remove_dir_all_idem (function)
macro_rules! Depcrate_utilremove_dir_all_idem {
() => {
// Module: crate::util
// Provides: {"remove_dir_all_idem"}
// Dependencies: {}
# [doc = " An idempotent version of the stdlib's remove_dir_all"] # [doc = " it is considered a success if the directory was not there."] fn remove_dir_all_idem (dir : & Path) -> std :: io :: Result < () > { match std :: fs :: remove_dir_all (dir) { Ok (_) => Ok (()) , Err (err) if err . kind () == io :: ErrorKind :: NotFound => Ok (()) , Err (err) => Err (err) , } }
};
}
