macro_rules! deps {
    () => {
        WIN32_ERROR!();
        HKEY!();
        PWSTR!();
    };
}

macro_rules! macro_13 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegEnumValueW (hkey : HKEY , dwindex : u32 , lpvaluename : PWSTR , lpcchvaluename : * mut u32 , lpreserved : * const u32 , lptype : * mut u32 , lpdata : * mut u8 , lpcbdata : * mut u32) -> WIN32_ERROR) ;
    };
}

macro_13!();