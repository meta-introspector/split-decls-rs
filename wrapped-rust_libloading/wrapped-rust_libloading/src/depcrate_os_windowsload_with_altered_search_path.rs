// Generated macro for LOAD_WITH_ALTERED_SEARCH_PATH (const)
macro_rules! Depcrate_os_windowsLOAD_WITH_ALTERED_SEARCH_PATH {
() => {
// Module: crate::os::windows
// Provides: {"LOAD_WITH_ALTERED_SEARCH_PATH"}
// Dependencies: {}
# [doc = " If `filename` specifies an absolute path, the system uses the alternate file search strategy"] # [doc = " discussed in the [Remarks section] to find associated executable modules that the specified"] # [doc = " module causes to be loaded."] # [doc = ""] # [doc = " If this value is used and `filename` specifies a relative path, the behaviour is undefined."] # [doc = ""] # [doc = " If this value is not used, or if `filename` does not specify a path, the system uses the"] # [doc = " standard search strategy discussed in the [Remarks section] to find associated executable"] # [doc = " modules that the specified module causes to be loaded."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] # [doc = ""] # [doc = " [Remarks]: https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#remarks"] pub const LOAD_WITH_ALTERED_SEARCH_PATH : LOAD_LIBRARY_FLAGS = 0x00000008 ;
};
}
