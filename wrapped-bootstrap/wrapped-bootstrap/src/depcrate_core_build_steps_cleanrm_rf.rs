// Generated macro for rm_rf (function)
macro_rules! Depcrate_core_build_steps_cleanrm_rf {
() => {
// Module: crate::core::build_steps::clean
// Provides: {"rm_rf"}
// Dependencies: {}
fn rm_rf (path : & Path) { match path . symlink_metadata () { Err (e) => { if e . kind () == ErrorKind :: NotFound { return ; } panic ! ("failed to get metadata for file {}: {}" , path . display () , e) ; } Ok (metadata) => { if ! metadata . file_type () . is_dir () { do_op (path , "remove file" , | p | match fs :: remove_file (p) { # [cfg (windows)] Err (e) if e . kind () == std :: io :: ErrorKind :: PermissionDenied && p . file_name () . and_then (std :: ffi :: OsStr :: to_str) == Some ("bootstrap.exe") => { eprintln ! ("WARNING: failed to delete '{}'." , p . display ()) ; Ok (()) } r => r , }) ; return ; } for file in t ! (fs :: read_dir (path)) { rm_rf (& t ! (file) . path ()) ; } do_op (path , "remove dir" , | p | match fs :: remove_dir (p) { # [cfg (windows)] Err (e) if e . kind () == ErrorKind :: DirectoryNotEmpty => Ok (()) , r => r , }) ; } } ; }
};
}
