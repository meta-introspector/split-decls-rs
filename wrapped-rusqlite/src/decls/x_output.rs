macro_rules! x_output {
    () => {
        unsafe extern "C" fn x_output (p_out : * mut c_void , data : * const c_void , len : c_int) -> c_int { if p_out . is_null () { return ffi :: SQLITE_MISUSE ; } let bytes : & [u8] = from_raw_parts (data as * const u8 , len as usize) ; let output = p_out as * mut & mut dyn Write ; match (* output) . write_all (bytes) { Ok (_) => ffi :: SQLITE_OK , Err (_) => ffi :: SQLITE_IOERR_WRITE , } }
    };
}

x_output!();