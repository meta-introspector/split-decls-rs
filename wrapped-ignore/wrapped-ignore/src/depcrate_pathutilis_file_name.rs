// Generated macro for is_file_name (function)
macro_rules! Depcrate_pathutilis_file_name {
() => {
// Module: crate::pathutil
// Provides: {"is_file_name"}
// Dependencies: {}
# [doc = " Returns true if this file path is just a file name. i.e., Its parent is"] # [doc = " the empty string."] # [cfg (not (unix))] pub (crate) fn is_file_name < P : AsRef < Path > > (path : P) -> bool { path . as_ref () . parent () . map (| p | p . as_os_str () . is_empty ()) . unwrap_or (false) }
};
}
