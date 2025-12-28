macro_rules! deps {
    () => {
        SYSTEMTIME!();
        BOOL!();
        TIME_ZONE_INFORMATION!();
    };
}

macro_rules! macro_549 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime (lptimezoneinformation : * const TIME_ZONE_INFORMATION , lpuniversaltime : * const SYSTEMTIME , lplocaltime : * mut SYSTEMTIME) -> BOOL) ;
    };
}

macro_549!()