macro_rules! macro_6 {
    () => {
        windows_link :: link ! ("combase.dll" "system" fn CoTaskMemFree (pv : * const core :: ffi :: c_void)) ;
    };
}

macro_6!();