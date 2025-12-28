macro_rules! macro_5 {
    () => {
        windows_link :: link ! ("combase.dll" "system" fn CoTaskMemAlloc (cb : usize) -> * mut core :: ffi :: c_void) ;
    };
}

macro_5!()