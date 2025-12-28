macro_rules! deps {
    () => {
        BOOL!();
        TIME_ZONE_INFORMATION!();
        DYNAMIC_TIME_ZONE_INFORMATION!();
    };
}

macro_rules! macro_547 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetTimeZoneInformationForYear (wyear : u16 , pdtzi : * const DYNAMIC_TIME_ZONE_INFORMATION , ptzi : * mut TIME_ZONE_INFORMATION) -> BOOL) ;
    };
}

macro_547!()