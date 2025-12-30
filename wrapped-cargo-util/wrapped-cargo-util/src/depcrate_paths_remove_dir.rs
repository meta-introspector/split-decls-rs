// Generated macro for _remove_dir (function)
macro_rules! Depcrate_paths_remove_dir {
() => {
// Module: crate::paths
// Provides: {"_remove_dir"}
// Dependencies: {}
fn _remove_dir (p : & Path) -> Result < () > { fs :: remove_dir (p) . with_context (| | format ! ("failed to remove directory `{}`" , p . display ())) ? ; Ok (()) }
};
}
