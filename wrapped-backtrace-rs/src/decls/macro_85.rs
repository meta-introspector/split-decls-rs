macro_rules! deps {
    () => {
        MODULEENTRY32W!();
        HANDLE!();
        BOOL!();
    };
}

macro_rules! macro_85 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn Module32NextW (hsnapshot : HANDLE , lpme : * mut MODULEENTRY32W) -> BOOL) ;
    };
}

macro_85!();