// Generated macro for LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR (const)
macro_rules! Depcrate_os_windowsLOAD_LIBRARY_SEARCH_DLL_LOAD_DIR {
() => {
// Module: crate::os::windows
// Provides: {"LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR"}
// Dependencies: {}
# [doc = " Directory that contains the DLL is temporarily added to the beginning of the list of"] # [doc = " directories that are searched for the DLL’s dependencies."] # [doc = ""] # [doc = " Directories in the standard search path are not searched."] # [doc = ""] # [doc = " The `filename` parameter must specify a fully qualified path. This value cannot be combined"] # [doc = " with [`LOAD_WITH_ALTERED_SEARCH_PATH`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR : LOAD_LIBRARY_FLAGS = 0x00000100 ;
};
}
