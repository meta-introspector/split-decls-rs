macro_rules! deps {
    () => {
        BOOL!();
        HANDLE!();
        PCSTR!();
    };
}

macro_rules! macro_29 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn OpenSemaphoreA (dwdesiredaccess : u32 , binherithandle : BOOL , lpname : PCSTR) -> HANDLE) ;
    };
}

macro_29!()