// Generated macro for xdg_config (function)
macro_rules! Depcrate_envxdg_config {
() => {
// Module: crate::env
// Provides: {"xdg_config"}
// Dependencies: {}
# [doc = " Returns the fully qualified path in the *xdg-home* directory (or equivalent in the home dir) to"] # [doc = " `file`, accessing `env_var(<name>)` to learn where these bases are."] # [doc = ""] # [doc = " Note that the `HOME` directory should ultimately come from [`home_dir()`] as it handles Windows"] # [doc = " correctly. The same can be achieved by using [`var()`] as `env_var`."] pub fn xdg_config (file : & str , env_var : & mut dyn FnMut (& str) -> Option < OsString >) -> Option < PathBuf > { env_var ("XDG_CONFIG_HOME") . map (| home | { let mut p = PathBuf :: from (home) ; p . push ("git") ; p . push (file) ; p }) . or_else (| | { env_var ("HOME") . map (| home | { let mut p = PathBuf :: from (home) ; p . push (".config") ; p . push ("git") ; p . push (file) ; p }) }) }
};
}
