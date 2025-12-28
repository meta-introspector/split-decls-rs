macro_rules! deps {
    () => {
        PCWSTR!();
        HANDLE!();
        SECURITY_ATTRIBUTES!();
        BOOL!();
    };
}

macro_rules! macro_106 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CreateEventW (lpeventattributes : * const SECURITY_ATTRIBUTES , bmanualreset : BOOL , binitialstate : BOOL , lpname : PCWSTR) -> HANDLE) ;
    };
}

macro_106!();