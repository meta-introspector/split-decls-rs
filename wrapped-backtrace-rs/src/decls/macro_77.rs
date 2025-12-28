macro_rules! deps {
    () => {
        HANDLE!();
        CREATE_TOOLHELP_SNAPSHOT_FLAGS!();
    };
}

macro_rules! macro_77 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CreateToolhelp32Snapshot (dwflags : CREATE_TOOLHELP_SNAPSHOT_FLAGS , th32processid : u32) -> HANDLE) ;
    };
}

macro_77!()