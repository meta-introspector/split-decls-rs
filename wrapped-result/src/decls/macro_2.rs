macro_rules! deps {
    () => {
        HRESULT!();
    };
}

macro_rules! macro_2 {
    () => {
        deps!();
        windows_link :: link ! ("oleaut32.dll" "system" fn GetErrorInfo (dwreserved : u32 , pperrinfo : * mut * mut core :: ffi :: c_void) -> HRESULT) ;
    };
}

macro_2!();