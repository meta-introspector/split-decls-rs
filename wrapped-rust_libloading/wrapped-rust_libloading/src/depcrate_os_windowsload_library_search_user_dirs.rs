// Generated macro for LOAD_LIBRARY_SEARCH_USER_DIRS (const)
macro_rules! Depcrate_os_windowsLOAD_LIBRARY_SEARCH_USER_DIRS {
() => {
// Module: crate::os::windows
// Provides: {"LOAD_LIBRARY_SEARCH_USER_DIRS"}
// Dependencies: {}
# [doc = "  Directories added using the `AddDllDirectory` or the `SetDllDirectory` function are searched"] # [doc = "  for the DLL and its dependencies."] # [doc = ""] # [doc = "  If more than one directory has been added, the order in which the directories are searched is"] # [doc = "  unspecified. Directories in the standard search path are not searched. This value cannot be"] # [doc = "  combined with [`LOAD_WITH_ALTERED_SEARCH_PATH`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SEARCH_USER_DIRS : LOAD_LIBRARY_FLAGS = 0x00000400 ;
};
}
