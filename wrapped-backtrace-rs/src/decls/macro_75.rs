macro_rules! deps {
    () => {
        HANDLE!();
        PAGE_PROTECTION_FLAGS!();
        SECURITY_ATTRIBUTES!();
        PCSTR!();
    };
}

macro_rules! macro_75 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn CreateFileMappingA (hfile : HANDLE , lpfilemappingattributes : * const SECURITY_ATTRIBUTES , flprotect : PAGE_PROTECTION_FLAGS , dwmaximumsizehigh : u32 , dwmaximumsizelow : u32 , lpname : PCSTR) -> HANDLE) ;
    };
}

macro_75!();