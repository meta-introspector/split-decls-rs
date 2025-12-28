macro_rules! deps {
    () => {
        PCSTR!();
        HMODULE!();
    };
}

macro_rules! macro_82 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn LoadLibraryA (lplibfilename : PCSTR) -> HMODULE) ;
    };
}

macro_82!();