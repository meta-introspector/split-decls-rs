// Generated macro for mounts (function)
macro_rules! Depcrate_fs_mounts_linuxmounts {
() => {
// Module: crate::fs::mounts::linux
// Provides: {"mounts"}
// Dependencies: {}
# [doc = " Get a list of all mounted filesystems"] pub fn mounts () -> Result < Vec < MountedFs > , Error > { Ok (MountList :: new () . map_err (Error :: IOError) ? . 0 . iter () . map (| mount | MountedFs { dest : mount . dest . clone () , fstype : mount . fstype . clone () , source : mount . source . to_string_lossy () . into () , }) . collect ()) }
};
}
