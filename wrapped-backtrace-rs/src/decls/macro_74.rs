macro_rules! deps {
    () => {
        BOOL!();
        HANDLE!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CloseHandle (hobject : HANDLE) -> BOOL) ;
    };
}

macro_74!();