macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! free_array {
    () => {
        deps!();
        pub (crate) unsafe extern "C" fn free_array (p : * mut c_void) { Rc :: decrement_strong_count (p as * const Vec < Value >) ; }
    };
}

free_array!();