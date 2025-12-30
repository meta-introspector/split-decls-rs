// Generated macro for RTLD_GLOBAL (const)
macro_rules! Depcrate_os_unix_constsRTLD_GLOBAL {
() => {
// Module: crate::os::unix::consts
// Provides: {"RTLD_GLOBAL"}
// Dependencies: {}
# [doc = " Make loaded symbols available for resolution globally."] # [doc = ""] # [doc = " The executable object file's symbols shall be made available for relocation processing of any"] # [doc = " other executable object file. In addition, calls to [`Library::get`] on `Library` obtained from"] # [doc = " [`Library::this`] allows executable object files loaded with this mode to be searched."] # [doc = ""] # [doc = " [`Library::this`]: crate::os::unix::Library::this"] # [doc = " [`Library::get`]: crate::os::unix::Library::get"] pub const RTLD_GLOBAL : c_int = posix :: RTLD_GLOBAL ;
};
}
