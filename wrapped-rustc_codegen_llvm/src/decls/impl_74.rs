macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl Drop for Linker < '_ > { fn drop (& mut self) { unsafe { llvm :: LLVMRustLinkerFree (& mut * (self . 0 as * mut _)) ; } } }
    };
}

impl_74!()