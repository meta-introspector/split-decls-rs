macro_rules! deps {
    () => {
        HANDLE!();
    };
}

macro_rules! macro_61 {
    () => {
        deps!();
        windows_link :: link ! ("dbghelp.dll" "system" fn SymAddrIncludeInlineTrace (hprocess : HANDLE , address : u64) -> u32) ;
    };
}

macro_61!()