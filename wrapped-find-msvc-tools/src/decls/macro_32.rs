macro_rules! deps {
    () => {
        WIN32_ERROR!();
        PWSTR!();
        HKEY!();
        FILETIME!();
    };
}

macro_rules! macro_32 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegEnumKeyExW (hkey : HKEY , dwindex : u32 , lpname : PWSTR , lpcchname : * mut u32 , lpreserved : * const u32 , lpclass : PWSTR , lpcchclass : * mut u32 , lpftlastwritetime : * mut FILETIME) -> WIN32_ERROR) ;
    };
}

macro_32!()