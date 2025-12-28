macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_IGNORE_CODE_AUTHZ_LEVEL {
    () => {
        deps!();
        # [doc = " Do not check AppLocker rules or apply Software Restriction Policies for the DLL."] # [doc = ""] # [doc = " This action applies only to the DLL being loaded and not to its dependencies. This value is"] # [doc = " recommended for use in setup programs that must run extracted DLLs during installation."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL : LOAD_LIBRARY_FLAGS = 0x00000010 ;
    };
}

LOAD_IGNORE_CODE_AUTHZ_LEVEL!();