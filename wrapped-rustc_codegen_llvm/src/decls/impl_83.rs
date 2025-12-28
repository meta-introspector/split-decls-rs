macro_rules! deps {
    () => {
        ModuleBuffer!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl Drop for ModuleBuffer { fn drop (& mut self) { unsafe { llvm :: LLVMRustModuleBufferFree (& mut * (self . 0 as * mut _)) ; } } }
    };
}

impl_83!()