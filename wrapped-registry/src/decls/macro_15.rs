macro_rules! deps {
    () => {
        REG_SAM_FLAGS!();
        WIN32_ERROR!();
        HANDLE!();
        HKEY!();
        PCWSTR!();
    };
}

macro_rules! macro_15 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegOpenKeyTransactedW (hkey : HKEY , lpsubkey : PCWSTR , uloptions : u32 , samdesired : REG_SAM_FLAGS , phkresult : * mut HKEY , htransaction : HANDLE , pextendedparemeter : * const core :: ffi :: c_void) -> WIN32_ERROR) ;
    };
}

macro_15!();