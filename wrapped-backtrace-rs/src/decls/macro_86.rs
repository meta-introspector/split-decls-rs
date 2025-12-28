macro_rules! deps {
    () => {
        HANDLE!();
        BOOL!();
    };
}

macro_rules! macro_86 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn ReleaseMutex (hmutex : HANDLE) -> BOOL) ;
    };
}

macro_86!();