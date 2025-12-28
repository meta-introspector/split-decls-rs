macro_rules! deps {
    () => {
        WIN32_ERROR!();
        REG_OPEN_CREATE_OPTIONS!();
        PCWSTR!();
        HKEY!();
        REG_SAM_FLAGS!();
        SECURITY_ATTRIBUTES!();
        REG_CREATE_KEY_DISPOSITION!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegCreateKeyExW (hkey : HKEY , lpsubkey : PCWSTR , reserved : u32 , lpclass : PCWSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : * const SECURITY_ATTRIBUTES , phkresult : * mut HKEY , lpdwdisposition : * mut REG_CREATE_KEY_DISPOSITION) -> WIN32_ERROR) ;
    };
}

macro_8!();