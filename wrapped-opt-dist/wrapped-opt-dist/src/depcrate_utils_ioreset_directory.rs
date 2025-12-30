// Generated macro for reset_directory (function)
macro_rules! Depcrate_utils_ioreset_directory {
() => {
// Module: crate::utils::io
// Provides: {"reset_directory"}
// Dependencies: {}
# [doc = " Delete and re-create the directory."] pub fn reset_directory (path : & Utf8Path) -> anyhow :: Result < () > { log :: info ! ("Resetting directory {path}") ; let _ = std :: fs :: remove_dir_all (path) ; std :: fs :: create_dir_all (path) ? ; Ok (()) }
};
}
