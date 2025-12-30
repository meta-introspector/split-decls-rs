// Generated macro for parse_rustc_version (function)
macro_rules! Depcrateparse_rustc_version {
() => {
// Module: crate
// Provides: {"parse_rustc_version"}
// Dependencies: {}
# [doc = " Parses the output of `rustc --version`."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `version` has an unexpected format."] # [allow (clippy :: missing_panics_doc)] pub fn parse_rustc_version (version : impl AsRef < str >) -> Result < (String , RustChannel) , String > { let matcher : safe_regex :: Matcher3 < _ > = safe_regex :: regex ! (br"(?:rustc )?([0-9]+\.[0-9]+\.[0-9]+)(?:(-beta)|(-nightly))?(?: .*)?") ; let (semver_bytes , beta , nightly) = matcher . match_slices (version . as_ref () . trim () . as_bytes ()) . ok_or_else (| | format ! ("failed parsing rustc version: '{}'" , version . as_ref ())) ? ; let semver = String :: from_utf8 (semver_bytes . to_vec ()) . unwrap () ; let channel = if ! beta . is_empty () { RustChannel :: Beta } else if ! nightly . is_empty () { RustChannel :: Nightly } else { RustChannel :: Stable } ; Ok ((semver , channel)) }
};
}
