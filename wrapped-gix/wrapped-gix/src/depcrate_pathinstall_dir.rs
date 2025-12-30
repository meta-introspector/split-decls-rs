// Generated macro for install_dir (function)
macro_rules! Depcrate_pathinstall_dir {
() => {
// Module: crate::path
// Provides: {"install_dir"}
// Dependencies: {}
pub (crate) fn install_dir () -> std :: io :: Result < PathBuf > { std :: env :: current_exe () . and_then (| exe | { exe . parent () . map (ToOwned :: to_owned) . ok_or_else (| | std :: io :: Error :: other ("no parent for current executable")) }) }
};
}
