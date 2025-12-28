macro_rules! macro_7 {
    () => {
        windows_link :: link ! ("kernel32.dll" "system" fn EncodePointer (ptr : * const core :: ffi :: c_void) -> * mut core :: ffi :: c_void) ;
    };
}

macro_7!();