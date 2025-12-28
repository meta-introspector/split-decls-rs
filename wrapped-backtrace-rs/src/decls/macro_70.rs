macro_rules! deps {
    () => {
        HANDLE!();
        PCWSTR!();
        BOOL!();
    };
}

macro_rules! macro_70 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymInitializeW (hprocess : HANDLE , usersearchpath : PCWSTR , finvadeprocess : BOOL) -> BOOL) ;
    };
}

macro_70!()