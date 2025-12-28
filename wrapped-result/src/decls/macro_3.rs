macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! macro_3 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetLastError () -> WIN32_ERROR) ;
    };
}

macro_3!()