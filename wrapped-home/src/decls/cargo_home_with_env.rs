macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! cargo_home_with_env {
    () => {
        deps!();
        # [doc = " Variant of `cargo_home` where the environment source is parameterized."] # [doc = ""] # [doc = " This is"] # [doc = " specifically to support in-process testing scenarios as environment"] # [doc = " variables and user home metadata are normally process global state. See the"] # [doc = " [`Env`] trait."] pub fn cargo_home_with_env (env : & dyn Env) -> io :: Result < PathBuf > { let cwd = env . current_dir () ? ; cargo_home_with_cwd_env (env , & cwd) }
    };
}

cargo_home_with_env!();