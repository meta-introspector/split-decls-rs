// Generated macro for installation_config (function)
macro_rules! Depcrate_envinstallation_config {
() => {
// Module: crate::env
// Provides: {"installation_config"}
// Dependencies: {}
# [doc = " Return the location at which installation specific git configuration file can be found, or `None`"] # [doc = " if the binary could not be executed or its results could not be parsed."] # [doc = ""] # [doc = " ### Performance"] # [doc = ""] # [doc = " This invokes the git binary which is slow on windows."] pub fn installation_config () -> Option < & 'static Path > { git :: install_config_path () . and_then (| p | crate :: try_from_byte_slice (p) . ok ()) }
};
}
