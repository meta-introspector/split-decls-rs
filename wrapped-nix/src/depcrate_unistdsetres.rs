// Generated macro for setres (module)
macro_rules! Depcrate_unistdsetres {
() => {
// Module: crate::unistd
// Provides: {"setres"}
// Dependencies: {}
# [cfg (any (linux_android , freebsdlike , target_os = "openbsd"))] mod setres { feature ! { #! [feature = "user"] use super :: { Gid , Uid } ; use crate :: errno :: Errno ; use crate :: Result ; # [doc = " Sets the real, effective, and saved uid."] # [doc = " ([see setresuid(2)](https://man7.org/linux/man-pages/man2/setresuid.2.html))"] # [doc = ""] # [doc = " * `ruid`: real user id"] # [doc = " * `euid`: effective user id"] # [doc = " * `suid`: saved user id"] # [doc = " * returns: Ok or libc error code."] # [doc = ""] # [doc = " Err is returned if the user doesn't have permission to set this UID."] # [inline] pub fn setresuid (ruid : Uid , euid : Uid , suid : Uid) -> Result < () > { let res = unsafe { libc :: setresuid (ruid . into () , euid . into () , suid . into ()) } ; Errno :: result (res) . map (drop) } # [doc = " Sets the real, effective, and saved gid."] # [doc = " ([see setresuid(2)](https://man7.org/linux/man-pages/man2/setresuid.2.html))"] # [doc = ""] # [doc = " * `rgid`: real group id"] # [doc = " * `egid`: effective group id"] # [doc = " * `sgid`: saved group id"] # [doc = " * returns: Ok or libc error code."] # [doc = ""] # [doc = " Err is returned if the user doesn't have permission to set this GID."] # [inline] pub fn setresgid (rgid : Gid , egid : Gid , sgid : Gid) -> Result < () > { let res = unsafe { libc :: setresgid (rgid . into () , egid . into () , sgid . into ()) } ; Errno :: result (res) . map (drop) } } }
};
}
