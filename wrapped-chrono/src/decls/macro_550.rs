macro_rules! deps {
    () => {
        TIME_ZONE_INFORMATION!();
        SYSTEMTIME!();
        BOOL!();
    };
}

macro_rules! macro_550 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn TzSpecificLocalTimeToSystemTime (lptimezoneinformation : * const TIME_ZONE_INFORMATION , lplocaltime : * const SYSTEMTIME , lpuniversaltime : * mut SYSTEMTIME) -> BOOL) ;
    };
}

macro_550!();