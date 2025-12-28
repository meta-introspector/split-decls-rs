macro_rules! deps {
    () => {
        HANDLE!();
        PWSTR!();
        BOOL!();
    };
}

macro_rules! macro_69 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymGetSearchPathW (hprocess : HANDLE , searchpatha : PWSTR , searchpathlength : u32) -> BOOL) ;
    };
}

macro_69!()