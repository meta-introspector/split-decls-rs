macro_rules! deps {
    () => {
        WIN32_ERROR!();
        HKEY!();
        PCWSTR!();
    };
}

macro_rules! macro_11 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegDeleteValueW (hkey : HKEY , lpvaluename : PCWSTR) -> WIN32_ERROR) ;
    };
}

macro_11!();