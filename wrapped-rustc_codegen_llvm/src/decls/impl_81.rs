macro_rules! deps {
    () => {
        ModuleBuffer!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl ModuleBuffer { pub (crate) fn new (m : & llvm :: Module) -> ModuleBuffer { ModuleBuffer (unsafe { llvm :: LLVMRustModuleBufferCreate (m) }) } }
    };
}

impl_81!();