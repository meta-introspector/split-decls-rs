macro_rules! deps {
    () => {
        HANDLE!();
        PCWSTR!();
        BOOL!();
    };
}

macro_rules! macro_73 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymSetSearchPathW (hprocess : HANDLE , searchpatha : PCWSTR) -> BOOL) ;
    };
}

macro_73!()