macro_rules! deps {
    () => {
        BOOL!();
        HANDLE!();
        IMAGEHLP_LINEW64!();
    };
}

macro_rules! macro_66 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymGetLineFromInlineContextW (hprocess : HANDLE , dwaddr : u64 , inlinecontext : u32 , qwmodulebaseaddress : u64 , pdwdisplacement : * mut u32 , line : * mut IMAGEHLP_LINEW64) -> BOOL) ;
    };
}

macro_66!();