// Generated macro for macro_298 (macro)
macro_rules! Depcrate_unistdmacro_298 {
() => {
// Module: crate::unistd
// Provides: {"macro_298"}
// Dependencies: {}
# [cfg (feature = "fs")] libc_bitflags ! { # [doc = " Options for access()"] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] pub struct AccessFlags : c_int { # [doc = " Test for existence of file."] F_OK ; # [doc = " Test for read permission."] R_OK ; # [doc = " Test for write permission."] W_OK ; # [doc = " Test for execute (search) permission."] X_OK ; } }
};
}
