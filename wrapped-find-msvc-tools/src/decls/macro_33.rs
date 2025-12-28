macro_rules! deps {
    () => {
        HKEY!();
        PCWSTR!();
        REG_SAM_FLAGS!();
        WIN32_ERROR!();
    };
}

macro_rules! macro_33 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegOpenKeyExW (hkey : HKEY , lpsubkey : PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : * mut HKEY) -> WIN32_ERROR) ;
    };
}

macro_33!();