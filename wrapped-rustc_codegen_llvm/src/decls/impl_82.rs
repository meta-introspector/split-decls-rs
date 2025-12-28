macro_rules! deps {
    () => {
        ModuleBuffer!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl ModuleBufferMethods for ModuleBuffer { fn data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustModuleBufferPtr (self . 0) ; let len = llvm :: LLVMRustModuleBufferLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }
    };
}

impl_82!()