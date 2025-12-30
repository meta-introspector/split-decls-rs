// Generated macro for parse_rustc_semver (function)
macro_rules! Depcrateparse_rustc_semver {
() => {
// Module: crate
// Provides: {"parse_rustc_semver"}
// Dependencies: {}
# [doc = " Gets the dotted-numeric version from the rustc version string."] # [doc = ""] # [doc = " Example: `\"1.53.0\"`"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `version` has an unexpected format."] pub fn parse_rustc_semver (version : impl AsRef < str >) -> Result < String , String > { let (semver , _channel) = parse_rustc_version (version) ? ; Ok (semver) }
};
}
