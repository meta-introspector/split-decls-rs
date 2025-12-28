macro_rules! deps {
    () => {
        WIN32_ERROR!();
        PCWSTR!();
        HKEY!();
        REG_VALUE_TYPE!();
    };
}

macro_rules! macro_34 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegQueryValueExW (hkey : HKEY , lpvaluename : PCWSTR , lpreserved : * const u32 , lptype : * mut REG_VALUE_TYPE , lpdata : * mut u8 , lpcbdata : * mut u32) -> WIN32_ERROR) ;
    };
}

macro_34!();