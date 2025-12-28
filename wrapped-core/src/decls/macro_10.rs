macro_rules! deps {
    () => {
        PCSTR!();
        HANDLE!();
        LOAD_LIBRARY_FLAGS!();
        HMODULE!();
    };
}

macro_rules! macro_10 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn LoadLibraryExA (lplibfilename : PCSTR , hfile : HANDLE , dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE) ;
    };
}

macro_10!()