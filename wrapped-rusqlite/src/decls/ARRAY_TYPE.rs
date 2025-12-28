macro_rules! ARRAY_TYPE {
    () => {
        pub (crate) const ARRAY_TYPE : * const c_char = c"rarray" . as_ptr () ;
    };
}

ARRAY_TYPE!();