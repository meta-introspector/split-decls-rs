macro_rules! deps {
    () => {
        HMODULE!();
        BOOL!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn FreeLibrary (hlibmodule : HMODULE) -> BOOL) ;
    };
}

macro_8!()