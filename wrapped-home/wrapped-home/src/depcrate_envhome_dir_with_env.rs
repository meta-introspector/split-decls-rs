// Generated macro for home_dir_with_env (function)
macro_rules! Depcrate_envhome_dir_with_env {
() => {
// Module: crate::env
// Provides: {"home_dir_with_env"}
// Dependencies: {}
# [doc = " Returns the path of the current user's home directory from [`Env::home_dir`]."] pub fn home_dir_with_env (env : & dyn Env) -> Option < PathBuf > { env . home_dir () }
};
}
