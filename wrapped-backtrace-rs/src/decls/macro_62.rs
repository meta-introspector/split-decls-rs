macro_rules! deps {
    () => {
        SYMBOL_INFOW!();
        BOOL!();
        HANDLE!();
    };
}

macro_rules! macro_62 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymFromAddrW (hprocess : HANDLE , address : u64 , displacement : * mut u64 , symbol : * mut SYMBOL_INFOW) -> BOOL) ;
    };
}

macro_62!()