// Generated macro for macro_56 (macro)
macro_rules! Depcrate_fcntlmacro_56 {
() => {
// Module: crate::fcntl
// Provides: {"macro_56"}
// Dependencies: {}
# [cfg (not (target_os = "redox"))] # [cfg (any (feature = "fs" , feature = "process" , feature = "user"))] libc_bitflags ! { # [doc = " Flags that control how the various *at syscalls behave."] # [cfg_attr (docsrs , doc (cfg (any (feature = "fs" , feature = "process"))))] pub struct AtFlags : c_int { # [allow (missing_docs)] # [doc (hidden)] AT_REMOVEDIR ; # [doc = " Used with [`linkat`](crate::unistd::linkat`) to create a link to a symbolic link's"] # [doc = " target, instead of to the symbolic link itself."] AT_SYMLINK_FOLLOW ; # [doc = " Used with functions like [`fstatat`](crate::sys::stat::fstatat`) to operate on a link"] # [doc = " itself, instead of the symbolic link's target."] AT_SYMLINK_NOFOLLOW ; # [doc = " Don't automount the terminal (\"basename\") component of pathname if it is a directory"] # [doc = " that is an automount point."] # [cfg (linux_android)] AT_NO_AUTOMOUNT ; # [doc = " If the provided path is an empty string, operate on the provided directory file"] # [doc = " descriptor instead."] # [cfg (any (linux_android , target_os = "freebsd" , target_os = "hurd"))] AT_EMPTY_PATH ; # [doc = " Used with [`faccessat`](crate::unistd::faccessat), the checks for accessibility are"] # [doc = " performed using the effective user and group IDs instead of the real user and group ID"] # [cfg (not (target_os = "android"))] AT_EACCESS ; } }
};
}
