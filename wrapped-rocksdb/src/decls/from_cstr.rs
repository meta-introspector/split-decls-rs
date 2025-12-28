macro_rules! from_cstr {
    () => {
        pub (crate) unsafe fn from_cstr (ptr : * const c_char) -> String { let cstr = unsafe { CStr :: from_ptr (ptr as * const _) } ; String :: from_utf8_lossy (cstr . to_bytes ()) . into_owned () }
    };
}

from_cstr!();