// Generated macro for is_collision_error (function)
macro_rules! Depcrate_symlinkis_collision_error {
() => {
// Module: crate::symlink
// Provides: {"is_collision_error"}
// Dependencies: {}
# [doc = " Return true if `err` indicates that a file collision happened, i.e. a symlink couldn't be created as the `link`"] # [doc = " already exists as filesystem object."] # [cfg (windows)] pub fn is_collision_error (err : & std :: io :: Error) -> bool { err . kind () == AlreadyExists || err . kind () == std :: io :: ErrorKind :: PermissionDenied }
};
}
