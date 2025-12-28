macro_rules! deps {
    () => {
        PCWSTR!();
    };
}

macro_rules! macro_95 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn lstrlenW (lpstring : PCWSTR) -> i32) ;
    };
}

macro_95!();