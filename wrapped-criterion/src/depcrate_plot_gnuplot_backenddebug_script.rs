// Generated macro for debug_script (function)
macro_rules! Depcrate_plot_gnuplot_backenddebug_script {
() => {
// Module: crate::plot::gnuplot_backend
// Provides: {"debug_script"}
// Dependencies: {}
fn debug_script (path : & Path , figure : & Figure) { if crate :: debug_enabled () { let mut script_path = path . to_path_buf () ; script_path . set_extension ("gnuplot") ; info ! ("Writing gnuplot script to {:?}" , script_path) ; let result = figure . save (script_path . as_path ()) ; if let Err (e) = result { error ! ("Failed to write debug output: {}" , e) ; } } }
};
}
