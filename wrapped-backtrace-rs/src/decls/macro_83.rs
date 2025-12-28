macro_rules! deps {
    () => {
        HANDLE!();
        FILE_MAP!();
        MEMORY_MAPPED_VIEW_ADDRESS!();
    };
}

macro_rules! macro_83 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn MapViewOfFile (hfilemappingobject : HANDLE , dwdesiredaccess : FILE_MAP , dwfileoffsethigh : u32 , dwfileoffsetlow : u32 , dwnumberofbytestomap : usize) -> MEMORY_MAPPED_VIEW_ADDRESS) ;
    };
}

macro_83!();