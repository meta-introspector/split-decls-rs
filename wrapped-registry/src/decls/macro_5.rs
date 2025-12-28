macro_rules! deps {
    () => {
        HEAP_FLAGS!();
        HANDLE!();
    };
}

macro_rules! macro_5 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn HeapAlloc (hheap : HANDLE , dwflags : HEAP_FLAGS , dwbytes : usize) -> * mut core :: ffi :: c_void) ;
    };
}

macro_5!();