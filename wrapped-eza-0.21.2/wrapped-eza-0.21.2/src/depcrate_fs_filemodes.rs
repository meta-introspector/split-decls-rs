// Generated macro for modes (module)
macro_rules! Depcrate_fs_filemodes {
() => {
// Module: crate::fs::file
// Provides: {"modes"}
// Dependencies: {}
# [doc = " More readable aliases for the permission bits exposed by libc."] # [allow (trivial_numeric_casts)] # [cfg (unix)] mod modes { pub type Mode = u32 ; pub const USER_READ : Mode = libc :: S_IRUSR as Mode ; pub const USER_WRITE : Mode = libc :: S_IWUSR as Mode ; pub const USER_EXECUTE : Mode = libc :: S_IXUSR as Mode ; pub const GROUP_READ : Mode = libc :: S_IRGRP as Mode ; pub const GROUP_WRITE : Mode = libc :: S_IWGRP as Mode ; pub const GROUP_EXECUTE : Mode = libc :: S_IXGRP as Mode ; pub const OTHER_READ : Mode = libc :: S_IROTH as Mode ; pub const OTHER_WRITE : Mode = libc :: S_IWOTH as Mode ; pub const OTHER_EXECUTE : Mode = libc :: S_IXOTH as Mode ; pub const STICKY : Mode = libc :: S_ISVTX as Mode ; pub const SETGID : Mode = libc :: S_ISGID as Mode ; pub const SETUID : Mode = libc :: S_ISUID as Mode ; }
};
}
