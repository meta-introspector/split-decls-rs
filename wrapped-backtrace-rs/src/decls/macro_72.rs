macro_rules! macro_72 {
    () => {
        windows_link :: link ! ("dbghelp.dll" "system" fn SymSetOptions (symoptions : u32) -> u32) ;
    };
}

macro_72!()