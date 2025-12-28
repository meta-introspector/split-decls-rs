macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! macro_67 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymGetModuleBase64 (hprocess : HANDLE , qwaddr : u64) -> u64) ;
    };
}

macro_67!();