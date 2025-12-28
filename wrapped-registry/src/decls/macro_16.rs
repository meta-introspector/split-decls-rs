macro_rules! deps {
    () => {
        WIN32_ERROR!();
        HKEY!();
        PWSTR!();
        FILETIME!();
    };
}

macro_rules! macro_16 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegQueryInfoKeyW (hkey : HKEY , lpclass : PWSTR , lpcchclass : * mut u32 , lpreserved : * const u32 , lpcsubkeys : * mut u32 , lpcbmaxsubkeylen : * mut u32 , lpcbmaxclasslen : * mut u32 , lpcvalues : * mut u32 , lpcbmaxvaluenamelen : * mut u32 , lpcbmaxvaluelen : * mut u32 , lpcbsecuritydescriptor : * mut u32 , lpftlastwritetime : * mut FILETIME) -> WIN32_ERROR) ;
    };
}

macro_16!();