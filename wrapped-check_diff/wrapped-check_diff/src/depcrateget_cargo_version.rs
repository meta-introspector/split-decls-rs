// Generated macro for get_cargo_version (function)
macro_rules! Depcrateget_cargo_version {
() => {
// Module: crate
// Provides: {"get_cargo_version"}
// Dependencies: {}
pub fn get_cargo_version () -> Result < String , CheckDiffError > { let Ok (command) = Command :: new ("cargo") . args (["--version"]) . output () else { return Err (CheckDiffError :: FailedCargoVersion ("Failed to obtain cargo version" ,)) ; } ; let cargo_version = std :: str :: from_utf8 (& command . stdout) ? . trim_end () ; return Ok (cargo_version . to_string ()) ; }
};
}
