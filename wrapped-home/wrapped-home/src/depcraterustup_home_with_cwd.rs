// Generated macro for rustup_home_with_cwd (function)
macro_rules! Depcraterustup_home_with_cwd {
() => {
// Module: crate
// Provides: {"rustup_home_with_cwd"}
// Dependencies: {}
# [doc = " Returns the storage directory used by rustup within `cwd`."] # [doc = " For more details, see [`rustup_home`](fn.rustup_home.html)."] pub fn rustup_home_with_cwd (cwd : & Path) -> io :: Result < PathBuf > { env :: rustup_home_with_cwd_env (& env :: OS_ENV , cwd) }
};
}
