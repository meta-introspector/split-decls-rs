macro_rules! deps {
    () => {
        HANDLE!();
        BOOL!();
    };
}

macro_rules! macro_105 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CloseHandle (hobject : HANDLE) -> BOOL) ;
    };
}

macro_105!()