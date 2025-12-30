// Generated macro for get_sysroot_dir (function)
macro_rules! Depcrate_utilget_sysroot_dir {
() => {
// Module: crate::util
// Provides: {"get_sysroot_dir"}
// Dependencies: {}
# [doc = " Determines where the sysroot of this execution is"] # [doc = ""] # [doc = " Either in a user-specified spot by an envar, or in a default cache location."] pub fn get_sysroot_dir () -> PathBuf { match std :: env :: var_os ("MIRI_SYSROOT") { Some (dir) => PathBuf :: from (dir) , None => { let user_dirs = directories :: ProjectDirs :: from ("org" , "rust-lang" , "miri") . unwrap () ; user_dirs . cache_dir () . to_owned () } } }
};
}
