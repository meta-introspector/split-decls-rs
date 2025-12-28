macro_rules! deps {
    () => {
        WIN32_ERROR!();
        HKEY!();
        PCWSTR!();
        REG_VALUE_TYPE!();
    };
}

macro_rules! macro_17 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegQueryValueExW (hkey : HKEY , lpvaluename : PCWSTR , lpreserved : * const u32 , lptype : * mut REG_VALUE_TYPE , lpdata : * mut u8 , lpcbdata : * mut u32) -> WIN32_ERROR) ;
    };
}

macro_17!()