macro_rules! deps {
    () => {
        HKEY!();
        WIN32_ERROR!();
    };
}

macro_rules! macro_31 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegCloseKey (hkey : HKEY) -> WIN32_ERROR) ;
    };
}

macro_31!()