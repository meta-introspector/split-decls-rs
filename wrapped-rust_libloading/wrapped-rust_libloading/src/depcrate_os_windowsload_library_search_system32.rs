// Generated macro for LOAD_LIBRARY_SEARCH_SYSTEM32 (const)
macro_rules! Depcrate_os_windowsLOAD_LIBRARY_SEARCH_SYSTEM32 {
() => {
// Module: crate::os::windows
// Provides: {"LOAD_LIBRARY_SEARCH_SYSTEM32"}
// Dependencies: {}
# [doc = " Search `%windows%\\system32` for the DLL and its dependencies."] # [doc = ""] # [doc = " Directories in the standard search path are not searched. This value cannot be combined with"] # [doc = " [`LOAD_WITH_ALTERED_SEARCH_PATH`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SEARCH_SYSTEM32 : LOAD_LIBRARY_FLAGS = 0x00000800 ;
};
}
