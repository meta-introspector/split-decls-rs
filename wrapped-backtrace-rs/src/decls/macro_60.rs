macro_rules! deps {
    () => {
        PREAD_PROCESS_MEMORY_ROUTINE64!();
        PFUNCTION_TABLE_ACCESS_ROUTINE64!();
        PTRANSLATE_ADDRESS_ROUTINE64!();
        HANDLE!();
        BOOL!();
        PGET_MODULE_BASE_ROUTINE64!();
        STACKFRAME_EX!();
    };
}

macro_rules! macro_60 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn StackWalkEx (machinetype : u32 , hprocess : HANDLE , hthread : HANDLE , stackframe : * mut STACKFRAME_EX , contextrecord : * mut core :: ffi :: c_void , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64 , functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64 , translateaddress : PTRANSLATE_ADDRESS_ROUTINE64 , flags : u32) -> BOOL) ;
    };
}

macro_60!()