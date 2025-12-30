// Generated macro for cargo_extra_flags (function)
macro_rules! Depcrate_utilcargo_extra_flags {
() => {
// Module: crate::util
// Provides: {"cargo_extra_flags"}
// Dependencies: {}
fn cargo_extra_flags () -> Vec < String > { let mut flags = Vec :: new () ; let config_flag = "--config" ; for arg in get_arg_flag_values (config_flag) { flags . push (config_flag . to_string ()) ; flags . push (arg) ; } let manifest_flag = "--manifest-path" ; if let Some (manifest) = get_arg_flag_value (manifest_flag) { flags . push (manifest_flag . to_string ()) ; flags . push (manifest) ; } flags }
};
}
