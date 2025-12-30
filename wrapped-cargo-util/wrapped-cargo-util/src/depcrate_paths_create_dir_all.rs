// Generated macro for _create_dir_all (function)
macro_rules! Depcrate_paths_create_dir_all {
() => {
// Module: crate::paths
// Provides: {"_create_dir_all"}
// Dependencies: {}
fn _create_dir_all (p : & Path) -> Result < () > { fs :: create_dir_all (p) . with_context (| | format ! ("failed to create directory `{}`" , p . display ())) ? ; Ok (()) }
};
}
