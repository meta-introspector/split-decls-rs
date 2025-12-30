// Generated macro for remove_file (function)
macro_rules! Depcrate_pathsremove_file {
() => {
// Module: crate::paths
// Provides: {"remove_file"}
// Dependencies: {}
# [doc = " Equivalent to [`std::fs::remove_file`] with better error messages."] # [doc = ""] # [doc = " If the file is readonly, this will attempt to change the permissions to"] # [doc = " force the file to be deleted."] # [doc = " On Windows, if the file is a symlink to a directory, this will attempt to remove"] # [doc = " the symlink itself."] pub fn remove_file < P : AsRef < Path > > (p : P) -> Result < () > { _remove_file (p . as_ref ()) }
};
}
