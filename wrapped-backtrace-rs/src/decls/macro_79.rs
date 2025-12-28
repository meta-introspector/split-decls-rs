macro_rules! macro_79 {
    () => {
        windows_link :: link ! ("kernel32.dll" "system" fn GetCurrentProcessId () -> u32) ;
    };
}

macro_79!();