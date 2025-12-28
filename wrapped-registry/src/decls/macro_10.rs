macro_rules! deps {
    () => {
        PCWSTR!();
        HKEY!();
        WIN32_ERROR!();
    };
}

macro_rules! macro_10 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegDeleteTreeW (hkey : HKEY , lpsubkey : PCWSTR) -> WIN32_ERROR) ;
    };
}

macro_10!()