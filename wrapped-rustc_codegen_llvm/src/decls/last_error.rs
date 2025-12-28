macro_rules! last_error {
    () => {
        pub (crate) fn last_error () -> Option < String > { unsafe { let cstr = LLVMRustGetLastError () ; if cstr . is_null () { None } else { let err = CStr :: from_ptr (cstr) . to_bytes () ; let err = String :: from_utf8_lossy (err) . to_string () ; libc :: free (cstr as * mut _) ; Some (err) } } }
    };
}

last_error!();