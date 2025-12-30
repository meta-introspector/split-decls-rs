// Generated macro for get_rust_lld_command (function)
macro_rules! Depcrateget_rust_lld_command {
() => {
// Module: crate
// Provides: {"get_rust_lld_command"}
// Dependencies: {}
# [doc = " Returns the command for invoking rust-lld with the correct flavor."] # [doc = " LLD only accepts the flavor argument at the first two arguments, so pass it there."] # [doc = ""] # [doc = " Exits on error."] fn get_rust_lld_command (current_exe_path : & Path) -> process :: Command { let rust_lld_path = get_rust_lld_path (current_exe_path) ; let mut command = process :: Command :: new (rust_lld_path) ; let flavor = get_lld_flavor (current_exe_path) . unwrap_or_exit_with ("executable has unexpected name") ; command . arg ("-flavor") ; command . arg (flavor) ; command . args (env :: args_os () . skip (1)) ; command }
};
}
