// Generated macro for path2bytes (function)
macro_rules! Depcrate_pathspath2bytes {
() => {
// Module: crate::paths
// Provides: {"path2bytes"}
// Dependencies: {}
# [doc = " Converts a path to UTF-8 bytes."] pub fn path2bytes (path : & Path) -> Result < & [u8] > { # [cfg (unix)] { use std :: os :: unix :: prelude :: * ; Ok (path . as_os_str () . as_bytes ()) } # [cfg (windows)] { match path . as_os_str () . to_str () { Some (s) => Ok (s . as_bytes ()) , None => Err (anyhow :: format_err ! ("invalid non-unicode path: {}" , path . display ())) , } } }
};
}
