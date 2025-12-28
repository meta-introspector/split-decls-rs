macro_rules! deps {
    () => {
        HANDLE!();
        BOOL!();
    };
}

macro_rules! PREAD_PROCESS_MEMORY_ROUTINE64 {
    () => {
        deps!();
        pub type PREAD_PROCESS_MEMORY_ROUTINE64 = Option < unsafe extern "system" fn (hprocess : HANDLE , qwbaseaddress : u64 , lpbuffer : * mut core :: ffi :: c_void , nsize : u32 , lpnumberofbytesread : * mut u32 ,) -> BOOL , > ;
    };
}

PREAD_PROCESS_MEMORY_ROUTINE64!()