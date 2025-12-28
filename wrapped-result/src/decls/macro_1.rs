macro_rules! deps {
    () => {
        FORMAT_MESSAGE_OPTIONS!();
        PWSTR!();
    };
}

macro_rules! macro_1 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn FormatMessageW (dwflags : FORMAT_MESSAGE_OPTIONS , lpsource : * const core :: ffi :: c_void , dwmessageid : u32 , dwlanguageid : u32 , lpbuffer : PWSTR , nsize : u32 , arguments : * const * const i8) -> u32) ;
    };
}

macro_1!();