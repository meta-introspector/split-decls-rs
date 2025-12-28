macro_rules! opt_str {
    () => {
        unsafe fn opt_str < 'a > (ptr : * const libc :: c_char) -> Option < & 'a str > { if ptr . is_null () { None } else { Some (str :: from_utf8 (CStr :: from_ptr (ptr) . to_bytes ()) . unwrap ()) } }
    };
}

opt_str!();