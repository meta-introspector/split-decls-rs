macro_rules! macro_41 {
    () => {
        # [cfg (feature = "fs")] libc_bitflags ! (# [doc = " Additional configuration flags for `fcntl`'s `F_SETFD`."] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] pub struct FdFlag : c_int { # [doc = " The file descriptor will automatically be closed during a successful `execve(2)`."] FD_CLOEXEC ; }) ;
    };
}

macro_41!()