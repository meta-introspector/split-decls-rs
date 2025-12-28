macro_rules! deps {
    () => {
        HANDLE!();
        BOOL!();
    };
}

macro_rules! macro_30 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn PeekNamedPipe (hnamedpipe : HANDLE , lpbuffer : * mut core :: ffi :: c_void , nbuffersize : u32 , lpbytesread : * mut u32 , lptotalbytesavail : * mut u32 , lpbytesleftthismessage : * mut u32) -> BOOL) ;
    };
}

macro_30!();