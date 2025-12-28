macro_rules! macro_236 {
    () => {
        # [cfg (feature = "fs")] libc_bitflags ! { # [doc = " Options for access()"] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] pub struct AccessFlags : c_int { # [doc = " Test for existence of file."] F_OK ; # [doc = " Test for read permission."] R_OK ; # [doc = " Test for write permission."] W_OK ; # [doc = " Test for execute (search) permission."] X_OK ; } }
    };
}

macro_236!()