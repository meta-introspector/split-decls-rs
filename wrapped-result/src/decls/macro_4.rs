macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! macro_4 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetProcessHeap () -> HANDLE) ;
    };
}

macro_4!();