macro_rules! set_value_name {
    () => {
        # [doc = " Safe wrapper for `LLVMSetValueName2` from a byte slice"] pub (crate) fn set_value_name (value : & Value , name : & [u8]) { unsafe { let data = name . as_c_char_ptr () ; LLVMSetValueName2 (value , data , name . len ()) ; } }
    };
}

set_value_name!()