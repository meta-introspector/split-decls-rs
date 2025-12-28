macro_rules! deps {
    () => {
        ThinBuffer!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Drop for ThinBuffer { fn drop (& mut self) { unsafe { llvm :: LLVMRustThinLTOBufferFree (& mut * (self . 0 as * mut _)) ; } } }
    };
}

impl_93!()