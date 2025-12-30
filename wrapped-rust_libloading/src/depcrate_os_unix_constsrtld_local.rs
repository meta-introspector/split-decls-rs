// Generated macro for RTLD_LOCAL (const)
macro_rules! Depcrate_os_unix_constsRTLD_LOCAL {
() => {
// Module: crate::os::unix::consts
// Provides: {"RTLD_LOCAL"}
// Dependencies: {}
# [doc = " Load symbols into an isolated namespace."] # [doc = ""] # [doc = " The executable object file's symbols shall not be made available for relocation processing of"] # [doc = " any other executable object file. This mode of operation is most appropriate for e.g. plugins."] pub const RTLD_LOCAL : c_int = posix :: RTLD_LOCAL ;
};
}
