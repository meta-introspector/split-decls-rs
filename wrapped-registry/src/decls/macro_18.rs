macro_rules! deps {
    () => {
        HKEY!();
        WIN32_ERROR!();
        PCWSTR!();
    };
}

macro_rules! macro_18 {
    () => {
        deps!();
        windows_link :: link ! ("advapi32.dll" "system" fn RegRenameKey (hkey : HKEY , lpsubkeyname : PCWSTR , lpnewkeyname : PCWSTR) -> WIN32_ERROR) ;
    };
}

macro_18!()