macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR {
    () => {
        deps!();
        # [doc = " Directory that contains the DLL is temporarily added to the beginning of the list of"] # [doc = " directories that are searched for the DLL’s dependencies."] # [doc = ""] # [doc = " Directories in the standard search path are not searched."] # [doc = ""] # [doc = " The `filename` parameter must specify a fully qualified path. This value cannot be combined"] # [doc = " with [`LOAD_WITH_ALTERED_SEARCH_PATH`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR : LOAD_LIBRARY_FLAGS = 0x00000100 ;
    };
}

LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR!()