macro_rules! deps {
    () => {
        REG_CREATE_KEY_DISPOSITION!();
        PCWSTR!();
        HKEY!();
        SECURITY_ATTRIBUTES!();
        REG_SAM_FLAGS!();
        REG_OPEN_CREATE_OPTIONS!();
        WIN32_ERROR!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegCreateKeyExW (hkey : HKEY , lpsubkey : PCWSTR , reserved : u32 , lpclass : PCWSTR , dwoptions : REG_OPEN_CREATE_OPTIONS , samdesired : REG_SAM_FLAGS , lpsecurityattributes : * const SECURITY_ATTRIBUTES , phkresult : * mut HKEY , lpdwdisposition : * mut REG_CREATE_KEY_DISPOSITION) -> WIN32_ERROR) ;
    };
}

macro_8!()