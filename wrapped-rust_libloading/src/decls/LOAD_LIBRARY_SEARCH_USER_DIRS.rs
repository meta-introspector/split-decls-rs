macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_SEARCH_USER_DIRS {
    () => {
        deps!();
        # [doc = "  Directories added using the `AddDllDirectory` or the `SetDllDirectory` function are searched"] # [doc = "  for the DLL and its dependencies."] # [doc = ""] # [doc = "  If more than one directory has been added, the order in which the directories are searched is"] # [doc = "  unspecified. Directories in the standard search path are not searched. This value cannot be"] # [doc = "  combined with [`LOAD_WITH_ALTERED_SEARCH_PATH`]."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SEARCH_USER_DIRS : LOAD_LIBRARY_FLAGS = 0x00000400 ;
    };
}

LOAD_LIBRARY_SEARCH_USER_DIRS!();