// Generated macro for split_file_name (function)
macro_rules! Depcrate_engine_customsplit_file_name {
() => {
// Module: crate::engine::custom
// Provides: {"split_file_name"}
// Dependencies: {}
fn split_file_name (path : & std :: path :: Path) -> (& std :: path :: Path , & OsStr) { if path_has_name (path) { (path . parent () . unwrap_or_else (| | std :: path :: Path :: new ("")) , path . file_name () . expect ("not called with `..`") ,) } else { (path , Default :: default ()) } }
};
}
