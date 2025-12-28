macro_rules! deps {
    () => {
        BOOL!();
        IMAGEHLP_LINEW64!();
        HANDLE!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymGetLineFromAddrW64 (hprocess : HANDLE , dwaddr : u64 , pdwdisplacement : * mut u32 , line : * mut IMAGEHLP_LINEW64) -> BOOL) ;
    };
}

macro_65!();