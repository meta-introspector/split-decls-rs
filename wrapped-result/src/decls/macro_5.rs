macro_rules! deps {
    () => {
        BOOL!();
        HEAP_FLAGS!();
        HANDLE!();
    };
}

macro_rules! macro_5 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn HeapFree (hheap : HANDLE , dwflags : HEAP_FLAGS , lpmem : * const core :: ffi :: c_void) -> BOOL) ;
    };
}

macro_5!()