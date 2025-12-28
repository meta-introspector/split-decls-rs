macro_rules! deps {
    () => {
        HANDLE!();
        HMODULE!();
        PCSTR!();
        LOAD_LIBRARY_FLAGS!();
    };
}

macro_rules! macro_6 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn LoadLibraryExA (lplibfilename : PCSTR , hfile : HANDLE , dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE) ;
    };
}

macro_6!();