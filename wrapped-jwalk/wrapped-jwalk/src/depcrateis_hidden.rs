// Generated macro for is_hidden (function)
macro_rules! Depcrateis_hidden {
() => {
// Module: crate
// Provides: {"is_hidden"}
// Dependencies: {}
fn is_hidden (file_name : & OsStr) -> bool { file_name . to_str () . map (| s | s . starts_with ('.')) . unwrap_or (false) }
};
}
