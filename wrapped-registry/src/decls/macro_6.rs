macro_rules! deps {
    () => {
        HANDLE!();
        HEAP_FLAGS!();
        BOOL!();
    };
}

macro_rules! macro_6 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn HeapFree (hheap : HANDLE , dwflags : HEAP_FLAGS , lpmem : * const core :: ffi :: c_void) -> BOOL) ;
    };
}

macro_6!();