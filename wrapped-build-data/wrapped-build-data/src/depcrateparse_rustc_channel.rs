// Generated macro for parse_rustc_channel (function)
macro_rules! Depcrateparse_rustc_channel {
() => {
// Module: crate
// Provides: {"parse_rustc_channel"}
// Dependencies: {}
# [doc = " Gets the channel from the rustc version string."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `version` has an unexpected format."] pub fn parse_rustc_channel (version : impl AsRef < str >) -> Result < RustChannel , String > { let (_semver , channel) = parse_rustc_version (version) ? ; Ok (channel) }
};
}
