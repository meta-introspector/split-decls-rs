macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! macro_8 {
    () => {
        deps!();
        windows_link :: link ! ("oleaut32.dll" "system" fn SetErrorInfo (dwreserved : u32 , perrinfo : * mut core :: ffi :: c_void) -> HRESULT) ;
    };
}

macro_8!()