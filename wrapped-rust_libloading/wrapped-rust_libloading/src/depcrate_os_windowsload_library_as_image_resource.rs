// Generated macro for LOAD_LIBRARY_AS_IMAGE_RESOURCE (const)
macro_rules! Depcrate_os_windowsLOAD_LIBRARY_AS_IMAGE_RESOURCE {
() => {
// Module: crate::os::windows
// Provides: {"LOAD_LIBRARY_AS_IMAGE_RESOURCE"}
// Dependencies: {}
# [doc = " Map the file into the process’ virtual address space as an image file."] # [doc = ""] # [doc = " The loader does not load the static imports or perform the other usual initialisation steps."] # [doc = " Use this flag when you want to load a DLL only to extract messages or resources from it."] # [doc = ""] # [doc = " Unless the application depends on the file having the in-memory layout of an image, this value"] # [doc = " should be used with either [`LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE`] or"] # [doc = " [`LOAD_LIBRARY_AS_DATAFILE`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE : LOAD_LIBRARY_FLAGS = 0x00000020 ;
};
}
