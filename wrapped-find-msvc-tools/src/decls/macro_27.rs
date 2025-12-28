macro_rules! deps {
    () => {
        FARPROC!();
        HMODULE!();
        PCSTR!();
    };
}

macro_rules! macro_27 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetProcAddress (hmodule : HMODULE , lpprocname : PCSTR) -> FARPROC) ;
    };
}

macro_27!()