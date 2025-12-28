macro_rules! deps {
    () => {
        BOOL!();
        HANDLE!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymQueryInlineTrace (hprocess : HANDLE , startaddress : u64 , startcontext : u32 , startretaddress : u64 , curaddress : u64 , curcontext : * mut u32 , curframeindex : * mut u32) -> BOOL) ;
    };
}

macro_71!()