macro_rules! error_message {
    () => {
        pub fn error_message (ptr : * const c_char) -> String { unsafe { let s = from_cstr (ptr) ; ffi :: rocksdb_free (ptr as * mut c_void) ; s } }
    };
}

error_message!()