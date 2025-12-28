macro_rules! add_global {
    () => {
        pub (crate) fn add_global < 'a > (llmod : & 'a Module , ty : & 'a Type , name_cstr : & CStr) -> & 'a Value { unsafe { LLVMAddGlobal (llmod , ty , name_cstr . as_ptr ()) } }
    };
}

add_global!();