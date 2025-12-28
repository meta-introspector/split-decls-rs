macro_rules! deps {
    () => {
        BOOL!();
        MEMORY_MAPPED_VIEW_ADDRESS!();
    };
}

macro_rules! macro_92 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn UnmapViewOfFile (lpbaseaddress : MEMORY_MAPPED_VIEW_ADDRESS) -> BOOL) ;
    };
}

macro_92!()