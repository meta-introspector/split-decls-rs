macro_rules! deps {
    () => {
        WIN32_ERROR!();
        HKEY!();
        PCWSTR!();
        REG_VALUE_TYPE!();
    };
}

macro_rules! macro_19 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegSetValueExW (hkey : HKEY , lpvaluename : PCWSTR , reserved : u32 , dwtype : REG_VALUE_TYPE , lpdata : * const u8 , cbdata : u32) -> WIN32_ERROR) ;
    };
}

macro_19!();