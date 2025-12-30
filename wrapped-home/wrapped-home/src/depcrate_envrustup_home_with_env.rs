// Generated macro for rustup_home_with_env (function)
macro_rules! Depcrate_envrustup_home_with_env {
() => {
// Module: crate::env
// Provides: {"rustup_home_with_env"}
// Dependencies: {}
# [doc = " Variant of `cargo_home_with_cwd` where the environment source is"] # [doc = " parameterized."] # [doc = ""] # [doc = " This is specifically to support in-process testing scenarios"] # [doc = " as environment variables and user home metadata are normally process global"] # [doc = " state. See the `OsEnv` trait."] pub fn rustup_home_with_env (env : & dyn Env) -> io :: Result < PathBuf > { let cwd = env . current_dir () ? ; rustup_home_with_cwd_env (env , & cwd) }
};
}
