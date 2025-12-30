// Generated macro for path_to_uri (function)
macro_rules! Depcrate_freedesktoppath_to_uri {
() => {
// Module: crate::freedesktop
// Provides: {"path_to_uri"}
// Dependencies: {}
fn path_to_uri (path : & Path) -> Result < Url , OpenError > { let path = path . canonicalize () . map_err (OpenError :: Io) ? ; Url :: from_file_path (path) . map_err (| _ | uri_to_open_error ()) }
};
}
