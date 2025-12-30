// Generated macro for LOAD_IGNORE_CODE_AUTHZ_LEVEL (const)
macro_rules! Depcrate_os_windowsLOAD_IGNORE_CODE_AUTHZ_LEVEL {
() => {
// Module: crate::os::windows
// Provides: {"LOAD_IGNORE_CODE_AUTHZ_LEVEL"}
// Dependencies: {}
# [doc = " Do not check AppLocker rules or apply Software Restriction Policies for the DLL."] # [doc = ""] # [doc = " This action applies only to the DLL being loaded and not to its dependencies. This value is"] # [doc = " recommended for use in setup programs that must run extracted DLLs during installation."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL : LOAD_LIBRARY_FLAGS = 0x00000010 ;
};
}
