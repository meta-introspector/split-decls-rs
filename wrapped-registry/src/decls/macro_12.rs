macro_rules! deps {
    () => {
        FILETIME!();
        WIN32_ERROR!();
        HKEY!();
        PWSTR!();
    };
}

macro_rules! macro_12 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegEnumKeyExW (hkey : HKEY , dwindex : u32 , lpname : PWSTR , lpcchname : * mut u32 , lpreserved : * const u32 , lpclass : PWSTR , lpcchclass : * mut u32 , lpftlastwritetime : * mut FILETIME) -> WIN32_ERROR) ;
    };
}

macro_12!();