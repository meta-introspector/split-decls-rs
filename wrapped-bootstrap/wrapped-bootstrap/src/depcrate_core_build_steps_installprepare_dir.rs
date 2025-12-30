// Generated macro for prepare_dir (function)
macro_rules! Depcrate_core_build_steps_installprepare_dir {
() => {
// Module: crate::core::build_steps::install
// Provides: {"prepare_dir"}
// Dependencies: {}
fn prepare_dir (destdir_env : & Option < PathBuf > , mut path : PathBuf , is_cygwin : bool) -> String { if let Some (destdir) = destdir_env { let without_destdir = path . clone () ; path . clone_from (destdir) ; for part in without_destdir . components () { if let Component :: Normal (s) = part { path . push (s) } } } if path . is_relative () { path = std :: env :: current_dir () . expect ("failed to get the current directory") . join (path) ; assert ! (path . is_absolute () , "could not make the path relative") ; } sanitize_sh (& path , is_cygwin) }
};
}
