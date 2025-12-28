macro_rules! deps {
    () => {
        HANDLE!();
        SECURITY_ATTRIBUTES!();
        BOOL!();
        PCSTR!();
    };
}

macro_rules! macro_76 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CreateMutexA (lpmutexattributes : * const SECURITY_ATTRIBUTES , binitialowner : BOOL , lpname : PCSTR) -> HANDLE) ;
    };
}

macro_76!();