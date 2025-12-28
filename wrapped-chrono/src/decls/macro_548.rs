macro_rules! deps {
    () => {
        SYSTEMTIME!();
        FILETIME!();
        BOOL!();
    };
}

macro_rules! macro_548 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn SystemTimeToFileTime (lpsystemtime : * const SYSTEMTIME , lpfiletime : * mut FILETIME) -> BOOL) ;
    };
}

macro_548!()