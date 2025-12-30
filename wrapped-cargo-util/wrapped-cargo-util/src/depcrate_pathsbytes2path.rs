// Generated macro for bytes2path (function)
macro_rules! Depcrate_pathsbytes2path {
() => {
// Module: crate::paths
// Provides: {"bytes2path"}
// Dependencies: {}
# [doc = " Converts UTF-8 bytes to a path."] pub fn bytes2path (bytes : & [u8]) -> Result < PathBuf > { # [cfg (unix)] { use std :: os :: unix :: prelude :: * ; Ok (PathBuf :: from (OsStr :: from_bytes (bytes))) } # [cfg (windows)] { use std :: str ; match str :: from_utf8 (bytes) { Ok (s) => Ok (PathBuf :: from (s)) , Err (..) => Err (anyhow :: format_err ! ("invalid non-unicode path")) , } } }
};
}
