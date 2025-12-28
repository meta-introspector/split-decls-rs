macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_SEARCH_DEFAULT_DIRS {
    () => {
        deps!();
        # [doc = " Search default directories when looking for the DLL and its dependencies."] # [doc = ""] # [doc = " This value is a combination of [`LOAD_LIBRARY_SEARCH_APPLICATION_DIR`],"] # [doc = " [`LOAD_LIBRARY_SEARCH_SYSTEM32`], and [`LOAD_LIBRARY_SEARCH_USER_DIRS`]. Directories in the"] # [doc = " standard search path are not searched. This value cannot be combined with"] # [doc = " [`LOAD_WITH_ALTERED_SEARCH_PATH`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS : LOAD_LIBRARY_FLAGS = 0x00001000 ;
    };
}

LOAD_LIBRARY_SEARCH_DEFAULT_DIRS!()