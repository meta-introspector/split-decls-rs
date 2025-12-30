// Generated macro for all_mounts (function)
macro_rules! Depcrate_fs_mountsall_mounts {
() => {
// Module: crate::fs::mounts
// Provides: {"all_mounts"}
// Dependencies: {}
pub (super) fn all_mounts () -> & 'static HashMap < PathBuf , MountedFs > { static ALL_MOUNTS : OnceLock < HashMap < PathBuf , MountedFs > > = OnceLock :: new () ; ALL_MOUNTS . get_or_init (| | { # [allow (unused_mut)] let mut mount_map : HashMap < PathBuf , MountedFs > = HashMap :: new () ; # [cfg (any (target_os = "linux" , target_os = "macos"))] if let Ok (mounts) = mounts () { for mount in mounts { mount_map . insert (mount . dest . clone () , mount) ; } } mount_map }) }
};
}
