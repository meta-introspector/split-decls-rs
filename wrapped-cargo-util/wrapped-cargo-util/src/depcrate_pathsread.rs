// Generated macro for read (function)
macro_rules! Depcrate_pathsread {
() => {
// Module: crate::paths
// Provides: {"read"}
// Dependencies: {}
# [doc = " Reads a file to a string."] # [doc = ""] # [doc = " Equivalent to [`std::fs::read_to_string`] with better error messages."] pub fn read (path : & Path) -> Result < String > { match String :: from_utf8 (read_bytes (path) ?) { Ok (s) => Ok (s) , Err (_) => anyhow :: bail ! ("path at `{}` was not valid utf-8" , path . display ()) , } }
};
}
