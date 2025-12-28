macro_rules! deps {
    () => {
        HKEY!();
        REG_SAM_FLAGS!();
        WIN32_ERROR!();
        PCWSTR!();
    };
}

macro_rules! macro_14 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegOpenKeyExW (hkey : HKEY , lpsubkey : PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : * mut HKEY) -> WIN32_ERROR) ;
    };
}

macro_14!();