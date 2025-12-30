// Generated macro for macro_285 (macro)
macro_rules! Depcrate_unistdmacro_285 {
() => {
// Module: crate::unistd
// Provides: {"macro_285"}
// Dependencies: {}
feature ! { #! [all (feature = "fs" , feature = "user")] # [doc = " Set the user identity used for filesystem checks per-thread."] # [doc = " On both success and failure, this call returns the previous filesystem user"] # [doc = " ID of the caller."] # [doc = ""] # [doc = " See also [setfsuid(2)](https://man7.org/linux/man-pages/man2/setfsuid.2.html)"] # [cfg (linux_android)] pub fn setfsuid (uid : Uid) -> Uid { let prev_fsuid = unsafe { libc :: setfsuid (uid . into ()) } ; Uid :: from_raw (prev_fsuid as uid_t) } # [doc = " Set the group identity used for filesystem checks per-thread."] # [doc = " On both success and failure, this call returns the previous filesystem group"] # [doc = " ID of the caller."] # [doc = ""] # [doc = " See also [setfsgid(2)](https://man7.org/linux/man-pages/man2/setfsgid.2.html)"] # [cfg (linux_android)] pub fn setfsgid (gid : Gid) -> Gid { let prev_fsgid = unsafe { libc :: setfsgid (gid . into ()) } ; Gid :: from_raw (prev_fsgid as gid_t) } }
};
}
