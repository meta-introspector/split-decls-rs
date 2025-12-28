macro_rules! deps {
    () => {
        HANDLE!();
        SECURITY_ATTRIBUTES!();
        PCWSTR!();
        BOOL!();
    };
}

macro_rules! macro_106 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CreateEventW (lpeventattributes : * const SECURITY_ATTRIBUTES , bmanualreset : BOOL , binitialstate : BOOL , lpname : PCWSTR) -> HANDLE) ;
    };
}

macro_106!()