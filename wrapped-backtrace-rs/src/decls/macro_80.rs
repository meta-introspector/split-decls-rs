macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! macro_80 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetCurrentThread () -> HANDLE) ;
    };
}

macro_80!()