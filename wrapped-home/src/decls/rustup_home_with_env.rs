macro_rules! deps {
    () => {
        Env!();
        OsEnv!();
    };
}

macro_rules! rustup_home_with_env {
    () => {
        deps!();
        # [doc = " Variant of `cargo_home_with_cwd` where the environment source is"] # [doc = " parameterized."] # [doc = ""] # [doc = " This is specifically to support in-process testing scenarios"] # [doc = " as environment variables and user home metadata are normally process global"] # [doc = " state. See the `OsEnv` trait."] pub fn rustup_home_with_env (env : & dyn Env) -> io :: Result < PathBuf > { let cwd = env . current_dir () ? ; rustup_home_with_cwd_env (env , & cwd) }
    };
}

rustup_home_with_env!()