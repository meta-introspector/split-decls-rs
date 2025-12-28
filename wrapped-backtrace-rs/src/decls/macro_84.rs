macro_rules! deps {
    () => {
        HANDLE!();
        BOOL!();
        MODULEENTRY32W!();
    };
}

macro_rules! macro_84 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn Module32FirstW (hsnapshot : HANDLE , lpme : * mut MODULEENTRY32W) -> BOOL) ;
    };
}

macro_84!()