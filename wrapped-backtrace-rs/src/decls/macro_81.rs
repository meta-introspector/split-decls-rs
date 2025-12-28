macro_rules! deps {
    () => {
        FARPROC!();
        PCSTR!();
        HMODULE!();
    };
}

macro_rules! macro_81 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetProcAddress (hmodule : HMODULE , lpprocname : PCSTR) -> FARPROC) ;
    };
}

macro_81!()