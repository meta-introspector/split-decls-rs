macro_rules! deps {
    () => {
        ThinBuffer!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl ThinBufferMethods for ThinBuffer { fn data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustThinLTOBufferPtr (self . 0) as * const _ ; let len = llvm :: LLVMRustThinLTOBufferLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }
    };
}

impl_92!();