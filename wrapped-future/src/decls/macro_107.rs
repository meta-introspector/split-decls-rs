macro_rules! deps {
    () => {
        BOOL!();
        HANDLE!();
    };
}

macro_rules! macro_107 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn SetEvent (hevent : HANDLE) -> BOOL) ;
    };
}

macro_107!();