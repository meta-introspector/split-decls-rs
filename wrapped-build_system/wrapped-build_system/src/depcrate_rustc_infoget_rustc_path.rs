// Generated macro for get_rustc_path (function)
macro_rules! Depcrate_rustc_infoget_rustc_path {
() => {
// Module: crate::rustc_info
// Provides: {"get_rustc_path"}
// Dependencies: {}
pub fn get_rustc_path () -> Option < PathBuf > { if let Ok (rustc) = std :: env :: var ("RUSTC") { return Some (PathBuf :: from (rustc)) ; } run_command (& [& "rustup" , & "which" , & "rustc"] , None) . ok () . map (| out | Path :: new (String :: from_utf8 (out . stdout) . unwrap () . trim ()) . to_path_buf ()) }
};
}
