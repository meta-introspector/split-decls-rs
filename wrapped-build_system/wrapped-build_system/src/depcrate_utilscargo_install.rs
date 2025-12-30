// Generated macro for cargo_install (function)
macro_rules! Depcrate_utilscargo_install {
() => {
// Module: crate::utils
// Provides: {"cargo_install"}
// Dependencies: {}
pub fn cargo_install (to_install : & str) -> Result < () , String > { let output = run_command (& [& "cargo" , & "install" , & "--list"] , None) ? ; let to_install_needle = format ! ("{to_install} ") ; if String :: from_utf8 (output . stdout) . unwrap () . lines () . any (| line | line . ends_with (':') && line . starts_with (& to_install_needle)) { return Ok (()) ; } if run_command_with_output (& [& "cargo" , & "install" , & to_install] , None) . is_err () { println ! ("Skipping installation of `{to_install}`") ; } Ok (()) }
};
}
