macro_rules! macro_40 {
    () => {
        # [cfg (any (linux_android , target_os = "freebsd"))] # [cfg (feature = "fs")] libc_bitflags ! (# [doc = " Additional flags for file sealing, which allows for limiting operations on a file."] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] pub struct SealFlag : c_int { # [doc = " Prevents further calls to `fcntl()` with `F_ADD_SEALS`."] F_SEAL_SEAL ; # [doc = " The file cannot be reduced in size."] F_SEAL_SHRINK ; # [doc = " The size of the file cannot be increased."] F_SEAL_GROW ; # [doc = " The file contents cannot be modified."] F_SEAL_WRITE ; # [doc = " The file contents cannot be modified, except via shared writable mappings that were"] # [doc = " created prior to the seal being set. Since Linux 5.1."] # [cfg (linux_android)] F_SEAL_FUTURE_WRITE ; }) ;
    };
}

macro_40!();