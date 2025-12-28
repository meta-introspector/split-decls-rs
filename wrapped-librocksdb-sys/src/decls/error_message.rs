macro_rules! error_message {
    () => {
        pub fn error_message (ptr : * const c_char) -> String { let c_str = unsafe { CStr :: from_ptr (ptr as * const _) } ; let s = str :: from_utf8 (c_str . to_bytes ()) . unwrap () . to_owned () ; unsafe { free (ptr as * mut c_void) ; } s }
    };
}

error_message!()