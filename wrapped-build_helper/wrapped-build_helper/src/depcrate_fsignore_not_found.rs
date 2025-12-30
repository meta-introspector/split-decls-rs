// Generated macro for ignore_not_found (function)
macro_rules! Depcrate_fsignore_not_found {
() => {
// Module: crate::fs
// Provides: {"ignore_not_found"}
// Dependencies: {}
# [doc = " Helper to ignore [`std::io::ErrorKind::NotFound`], but still propagate other"] # [doc = " [`std::io::ErrorKind`]s."] pub fn ignore_not_found < Op > (mut op : Op) -> io :: Result < () > where Op : FnMut () -> io :: Result < () > , { match op () { Ok (()) => Ok (()) , Err (e) if e . kind () == io :: ErrorKind :: NotFound => Ok (()) , Err (e) => Err (e) , } }
};
}
