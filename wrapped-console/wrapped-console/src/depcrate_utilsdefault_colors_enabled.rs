// Generated macro for default_colors_enabled (function)
macro_rules! Depcrate_utilsdefault_colors_enabled {
() => {
// Module: crate::utils
// Provides: {"default_colors_enabled"}
// Dependencies: {}
fn default_colors_enabled (out : & Term) -> bool { (out . features () . colors_supported () && & env :: var ("CLICOLOR") . unwrap_or_else (| _ | "1" . into ()) != "0") || & env :: var ("CLICOLOR_FORCE") . unwrap_or_else (| _ | "0" . into ()) != "0" }
};
}
