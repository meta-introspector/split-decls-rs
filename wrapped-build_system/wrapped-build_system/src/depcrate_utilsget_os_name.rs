// Generated macro for get_os_name (function)
macro_rules! Depcrate_utilsget_os_name {
() => {
// Module: crate::utils
// Provides: {"get_os_name"}
// Dependencies: {}
pub fn get_os_name () -> Result < String , String > { let output = run_command (& [& "uname"] , None) ? ; let name = std :: str :: from_utf8 (& output . stdout) . unwrap_or ("") . trim () . to_string () ; if ! name . is_empty () { Ok (name) } else { Err ("Failed to retrieve the OS name" . to_string ()) } }
};
}
