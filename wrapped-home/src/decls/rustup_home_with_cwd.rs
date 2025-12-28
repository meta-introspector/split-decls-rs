macro_rules! rustup_home_with_cwd {
    () => {
        # [doc = " Returns the storage directory used by rustup within `cwd`."] # [doc = " For more details, see [`rustup_home`](fn.rustup_home.html)."] pub fn rustup_home_with_cwd (cwd : & Path) -> io :: Result < PathBuf > { env :: rustup_home_with_cwd_env (& env :: OS_ENV , cwd) }
    };
}

rustup_home_with_cwd!()