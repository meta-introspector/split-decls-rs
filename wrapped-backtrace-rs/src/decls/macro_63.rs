macro_rules! deps {
    () => {
        HANDLE!();
        SYMBOL_INFOW!();
        BOOL!();
    };
}

macro_rules! macro_63 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymFromInlineContextW (hprocess : HANDLE , address : u64 , inlinecontext : u32 , displacement : * mut u64 , symbol : * mut SYMBOL_INFOW) -> BOOL) ;
    };
}

macro_63!()