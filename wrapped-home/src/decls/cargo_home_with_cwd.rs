macro_rules! cargo_home_with_cwd {
    () => {
        # [doc = " Returns the storage directory used by Cargo within `cwd`."] # [doc = " For more details, see [`cargo_home`](fn.cargo_home.html)."] pub fn cargo_home_with_cwd (cwd : & Path) -> io :: Result < PathBuf > { env :: cargo_home_with_cwd_env (& env :: OS_ENV , cwd) }
    };
}

cargo_home_with_cwd!()