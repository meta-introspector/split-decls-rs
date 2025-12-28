macro_rules! set_initializer {
    () => {
        pub (crate) fn set_initializer (llglobal : & Value , constant_val : & Value) { unsafe { LLVMSetInitializer (llglobal , constant_val) ; } }
    };
}

set_initializer!();