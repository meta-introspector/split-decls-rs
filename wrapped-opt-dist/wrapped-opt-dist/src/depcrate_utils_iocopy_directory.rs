// Generated macro for copy_directory (function)
macro_rules! Depcrate_utils_iocopy_directory {
() => {
// Module: crate::utils::io
// Provides: {"copy_directory"}
// Dependencies: {}
pub fn copy_directory (src : & Utf8Path , dst : & Utf8Path) -> anyhow :: Result < () > { log :: info ! ("Copying directory {src} to {dst}") ; fs_extra :: dir :: copy (src , dst , & CopyOptions :: default () . copy_inside (true)) ? ; Ok (()) }
};
}
