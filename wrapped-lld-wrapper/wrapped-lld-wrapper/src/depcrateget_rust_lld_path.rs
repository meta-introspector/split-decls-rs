// Generated macro for get_rust_lld_path (function)
macro_rules! Depcrateget_rust_lld_path {
() => {
// Module: crate
// Provides: {"get_rust_lld_path"}
// Dependencies: {}
# [doc = " Returns the path to rust-lld in the parent directory."] # [doc = ""] # [doc = " Exits if the parent directory cannot be determined."] fn get_rust_lld_path (current_exe_path : & Path) -> PathBuf { let mut rust_lld_exe_name = "rust-lld" . to_owned () ; rust_lld_exe_name . push_str (EXE_SUFFIX) ; let mut rust_lld_path = current_exe_path . parent () . unwrap_or_exit_with ("directory containing current executable could not be determined") . parent () . unwrap_or_exit_with ("parent directory could not be determined") . to_owned () ; rust_lld_path . push (rust_lld_exe_name) ; rust_lld_path }
};
}
