macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! home_dir_with_env {
    () => {
        deps!();
        # [doc = " Returns the path of the current user's home directory from [`Env::home_dir`]."] pub fn home_dir_with_env (env : & dyn Env) -> Option < PathBuf > { env . home_dir () }
    };
}

home_dir_with_env!()