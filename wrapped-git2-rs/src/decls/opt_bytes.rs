macro_rules! opt_bytes {
    () => {
        unsafe fn opt_bytes < 'a , T > (_anchor : & 'a T , c : * const libc :: c_char) -> Option < & 'a [u8] > { if c . is_null () { None } else { Some (CStr :: from_ptr (c) . to_bytes ()) } }
    };
}

opt_bytes!();