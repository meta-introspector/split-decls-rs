// Generated macro for get_cargo (function)
macro_rules! Depcrate_utilsget_cargo {
() => {
// Module: crate::utils
// Provides: {"get_cargo"}
// Dependencies: {}
# [cfg (feature = "_cargo_insta_internal")] pub fn get_cargo () -> std :: ffi :: OsString { let cargo = env :: var_os ("CARGO") ; let cargo = cargo . as_deref () . unwrap_or_else (| | std :: ffi :: OsStr :: new ("cargo")) ; cargo . to_os_string () }
};
}
