// Generated macro for get_ld_library_path (function)
macro_rules! Depcrateget_ld_library_path {
() => {
// Module: crate
// Provides: {"get_ld_library_path"}
// Dependencies: {}
pub fn get_ld_library_path (dir : & Path) -> Result < String , CheckDiffError > { let Ok (command) = Command :: new ("rustc") . current_dir (dir) . args (["--print" , "sysroot"]) . output () else { return Err (CheckDiffError :: FailedCommand ("Error getting sysroot")) ; } ; let sysroot = std :: str :: from_utf8 (& command . stdout) ? . trim_end () ; let ld_lib_path = format ! ("{}/lib" , sysroot) ; return Ok (ld_lib_path) ; }
};
}
