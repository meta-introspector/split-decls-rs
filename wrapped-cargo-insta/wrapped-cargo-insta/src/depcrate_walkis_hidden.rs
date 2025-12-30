// Generated macro for is_hidden (function)
macro_rules! Depcrate_walkis_hidden {
() => {
// Module: crate::walk
// Provides: {"is_hidden"}
// Dependencies: {}
fn is_hidden (entry : & DirEntry) -> bool { entry . file_name () . to_str () . map (| s | s . starts_with ('.')) . unwrap_or (false) }
};
}
