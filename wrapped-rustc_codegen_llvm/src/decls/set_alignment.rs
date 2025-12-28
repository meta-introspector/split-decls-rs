macro_rules! set_alignment {
    () => {
        pub (crate) fn set_alignment (llglobal : & Value , align : Align) { unsafe { ffi :: LLVMSetAlignment (llglobal , align . bytes () as c_uint) ; } }
    };
}

set_alignment!()