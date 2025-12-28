macro_rules! deps {
    () => {
        ThinData!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Drop for ThinData { fn drop (& mut self) { unsafe { llvm :: LLVMRustFreeThinLTOData (& mut * (self . 0 as * mut _)) ; } } }
    };
}

impl_87!();