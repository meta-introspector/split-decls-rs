macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_SEARCH_SYSTEM32 {
    () => {
        deps!();
        # [doc = " Search `%windows%\\system32` for the DLL and its dependencies."] # [doc = ""] # [doc = " Directories in the standard search path are not searched. This value cannot be combined with"] # [doc = " [`LOAD_WITH_ALTERED_SEARCH_PATH`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SEARCH_SYSTEM32 : LOAD_LIBRARY_FLAGS = 0x00000800 ;
    };
}

LOAD_LIBRARY_SEARCH_SYSTEM32!()