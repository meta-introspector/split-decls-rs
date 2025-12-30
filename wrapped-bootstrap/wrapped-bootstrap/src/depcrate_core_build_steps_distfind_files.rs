// Generated macro for find_files (function)
macro_rules! Depcrate_core_build_steps_distfind_files {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"find_files"}
// Dependencies: {}
fn find_files (files : & [& str] , path : & [PathBuf]) -> Vec < PathBuf > { let mut found = Vec :: with_capacity (files . len ()) ; for file in files { let file_path = path . iter () . map (| dir | dir . join (file)) . find (| p | p . exists ()) ; if let Some (file_path) = file_path { found . push (file_path) ; } else { panic ! ("Could not find '{file}' in {path:?}") ; } } found }
};
}
