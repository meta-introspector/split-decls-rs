macro_rules! set_section {
    () => {
        pub (crate) fn set_section (llglobal : & Value , section_name : & CStr) { unsafe { LLVMSetSection (llglobal , section_name . as_ptr ()) ; } }
    };
}

set_section!()