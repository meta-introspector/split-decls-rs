// Generated macro for if_exists (function)
macro_rules! Depcrate_htmlif_exists {
() => {
// Module: crate::html
// Provides: {"if_exists"}
// Dependencies: {}
fn if_exists (output_directory : & Path , path : & Path) -> Option < String > { let report_path = path . join ("report/index.html") ; if PathBuf :: from (output_directory) . join (& report_path) . is_file () { Some (report_path . to_string_lossy () . to_string ()) } else { None } }
};
}
