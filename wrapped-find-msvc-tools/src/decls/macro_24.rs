macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! macro_24 {
    () => {
        deps!();
        windows_link :: link ! ("ole32.dll" "system" fn CoInitializeEx (pvreserved : * const core :: ffi :: c_void , dwcoinit : u32) -> HRESULT) ;
    };
}

macro_24!()