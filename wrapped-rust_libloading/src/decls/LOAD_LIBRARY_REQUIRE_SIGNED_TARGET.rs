macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! LOAD_LIBRARY_REQUIRE_SIGNED_TARGET {
    () => {
        deps!();
        # [doc = " Specifies that the digital signature of the binary image must be checked at load time."] # [doc = ""] # [doc = " See [flag documentation on MSDN](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw#parameters)."] pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET : LOAD_LIBRARY_FLAGS = 0x00000080 ;
    };
}

LOAD_LIBRARY_REQUIRE_SIGNED_TARGET!()