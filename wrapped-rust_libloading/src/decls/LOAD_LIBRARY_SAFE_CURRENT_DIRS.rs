macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_SAFE_CURRENT_DIRS {
    () => {
        deps!();
        # [doc = " Allow loading a DLL for execution from the current directory only if it is under a directory in"] # [doc = " the Safe load list."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS : LOAD_LIBRARY_FLAGS = 0x00002000 ;
    };
}

LOAD_LIBRARY_SAFE_CURRENT_DIRS!();