macro_rules! deps {
    () => {
        HMODULE!();
        FARPROC!();
        PCSTR!();
    };
}

macro_rules! macro_9 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetProcAddress (hmodule : HMODULE , lpprocname : PCSTR) -> FARPROC) ;
    };
}

macro_9!();