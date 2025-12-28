macro_rules! x_input {
    () => {
        unsafe extern "C" fn x_input (p_in : * mut c_void , data : * mut c_void , len : * mut c_int) -> c_int { if p_in . is_null () { return ffi :: SQLITE_MISUSE ; } let bytes : & mut [u8] = from_raw_parts_mut (data as * mut u8 , * len as usize) ; let input = p_in as * mut & mut dyn Read ; match (* input) . read (bytes) { Ok (n) => { * len = n as i32 ; ffi :: SQLITE_OK } Err (_) => ffi :: SQLITE_IOERR_READ , } }
    };
}

x_input!()