macro_rules! deps {
    () => {
        LOAD_LIBRARY_FLAGS!();
        PCSTR!();
        HMODULE!();
        HANDLE!();
    };
}

macro_rules! macro_10 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn LoadLibraryExA (lplibfilename : PCSTR , hfile : HANDLE , dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE) ;
    };
}

macro_10!();