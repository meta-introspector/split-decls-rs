// Generated macro for make_lockfile_copy (function)
macro_rules! Depcrate_cargo_config_filemake_lockfile_copy {
() => {
// Module: crate::cargo_config_file
// Provides: {"make_lockfile_copy"}
// Dependencies: {}
pub (crate) fn make_lockfile_copy (lockfile_path : & Utf8Path ,) -> Option < (temp_dir :: TempDir , Utf8PathBuf) > { let temp_dir = temp_dir :: TempDir :: with_prefix ("rust-analyzer") . ok () ? ; let target_lockfile = temp_dir . path () . join ("Cargo.lock") . try_into () . ok () ? ; match std :: fs :: copy (lockfile_path , & target_lockfile) { Ok (_) => { tracing :: debug ! ("Copied lock file from `{}` to `{}`" , lockfile_path , target_lockfile) ; Some ((temp_dir , target_lockfile)) } Err (e) if e . kind () == std :: io :: ErrorKind :: NotFound => Some ((temp_dir , target_lockfile)) , Err (e) => { tracing :: warn ! ("Failed to copy lock file from `{lockfile_path}` to `{target_lockfile}`: {e}" ,) ; None } } }
};
}
