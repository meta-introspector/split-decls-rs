macro_rules! deps {
    () => {
        PGET_MODULE_BASE_ROUTINE64!();
        HANDLE!();
        PTRANSLATE_ADDRESS_ROUTINE64!();
        PFUNCTION_TABLE_ACCESS_ROUTINE64!();
        PREAD_PROCESS_MEMORY_ROUTINE64!();
        STACKFRAME64!();
        BOOL!();
    };
}

macro_rules! macro_59 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn StackWalk64 (machinetype : u32 , hprocess : HANDLE , hthread : HANDLE , stackframe : * mut STACKFRAME64 , contextrecord : * mut core :: ffi :: c_void , readmemoryroutine : PREAD_PROCESS_MEMORY_ROUTINE64 , functiontableaccessroutine : PFUNCTION_TABLE_ACCESS_ROUTINE64 , getmodulebaseroutine : PGET_MODULE_BASE_ROUTINE64 , translateaddress : PTRANSLATE_ADDRESS_ROUTINE64) -> BOOL) ;
    };
}

macro_59!();