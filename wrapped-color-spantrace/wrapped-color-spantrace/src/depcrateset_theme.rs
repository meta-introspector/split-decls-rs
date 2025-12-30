// Generated macro for set_theme (function)
macro_rules! Depcrateset_theme {
() => {
// Module: crate
// Provides: {"set_theme"}
// Dependencies: {}
# [doc = " Sets the global theme."] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " This can only be set once and otherwise fails."] # [doc = ""] # [doc = " **Note:** `colorize` sets the global theme implicitly, if it was not set already. So calling `colorize` and then `set_theme` fails"] pub fn set_theme (theme : Theme) -> Result < () , InstallThemeError > { THEME . set (theme) . map_err (| _ | InstallThemeError) }
};
}
