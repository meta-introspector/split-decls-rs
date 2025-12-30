// Generated macro for wsl_path (function)
macro_rules! Depcrate_unixwsl_path {
() => {
// Module: crate::unix
// Provides: {"wsl_path"}
// Dependencies: {}
fn wsl_path < T : AsRef < OsStr > > (path : T) -> OsString { fn path_relative_to_current_dir < T : AsRef < OsStr > > (path : T) -> Option < PathBuf > { let path = Path :: new (& path) ; if path . is_relative () { return None ; } let base = env :: current_dir () . ok () ? ; pathdiff :: diff_paths (path , base) } match path_relative_to_current_dir (& path) { None => OsString :: from (& path) , Some (relative) => OsString :: from (relative) , } }
};
}
