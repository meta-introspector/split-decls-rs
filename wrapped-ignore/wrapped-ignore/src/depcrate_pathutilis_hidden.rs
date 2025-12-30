// Generated macro for is_hidden (function)
macro_rules! Depcrate_pathutilis_hidden {
() => {
// Module: crate::pathutil
// Provides: {"is_hidden"}
// Dependencies: {}
# [doc = " Returns true if and only if this entry is considered to be hidden."] # [doc = ""] # [doc = " This only returns true if the base name of the path starts with a `.`."] # [cfg (not (any (unix , windows)))] pub (crate) fn is_hidden (dent : & DirEntry) -> bool { if let Some (name) = file_name (dent . path ()) { name . to_str () . map (| s | s . starts_with (".")) . unwrap_or (false) } else { false } }
};
}
