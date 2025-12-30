// Generated macro for find_exe (function)
macro_rules! Depcratefind_exe {
() => {
// Module: crate
// Provides: {"find_exe"}
// Dependencies: {}
fn find_exe (path : & Path) -> PathBuf { env :: split_paths (& env :: var_os ("PATH") . unwrap_or_default ()) . map (| p | p . join (path)) . find (| p | fs :: metadata (p) . is_ok ()) . unwrap_or_else (| | path . to_owned ()) }
};
}
