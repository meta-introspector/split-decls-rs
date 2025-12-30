// Generated macro for LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE (const)
macro_rules! Depcrate_os_windowsLOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE {
() => {
// Module: crate::os::windows
// Provides: {"LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE"}
// Dependencies: {}
# [doc = " Map the file into the calling process’ virtual address space as if it were a data file."] # [doc = ""] # [doc = " Similar to [`LOAD_LIBRARY_AS_DATAFILE`], except that the DLL file is opened with exclusive"] # [doc = " write access for the calling process. Other processes cannot open the DLL file for write access"] # [doc = " while it is in use. However, the DLL can still be opened by other processes."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE : LOAD_LIBRARY_FLAGS = 0x00000040 ;
};
}
