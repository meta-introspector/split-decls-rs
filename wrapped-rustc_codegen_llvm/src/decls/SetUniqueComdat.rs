macro_rules! SetUniqueComdat {
    () => {
        pub (crate) fn SetUniqueComdat (llmod : & Module , val : & Value) { let name_buf = get_value_name (val) ; let name = CString :: from_vec_with_nul (name_buf) . or_else (| buf | CString :: new (buf . into_bytes ())) . unwrap () ; set_comdat (llmod , val , & name) ; }
    };
}

SetUniqueComdat!()