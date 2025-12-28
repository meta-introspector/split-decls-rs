macro_rules! macro_68 {
    () => {
        windows_link :: link ! ("dbghelp.dll" "system" fn SymGetOptions () -> u32) ;
    };
}

macro_68!();