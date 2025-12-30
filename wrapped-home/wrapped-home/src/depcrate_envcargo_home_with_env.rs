// Generated macro for cargo_home_with_env (function)
macro_rules! Depcrate_envcargo_home_with_env {
() => {
// Module: crate::env
// Provides: {"cargo_home_with_env"}
// Dependencies: {}
# [doc = " Variant of `cargo_home` where the environment source is parameterized."] # [doc = ""] # [doc = " This is"] # [doc = " specifically to support in-process testing scenarios as environment"] # [doc = " variables and user home metadata are normally process global state. See the"] # [doc = " [`Env`] trait."] pub fn cargo_home_with_env (env : & dyn Env) -> io :: Result < PathBuf > { let cwd = env . current_dir () ? ; cargo_home_with_cwd_env (env , & cwd) }
};
}
