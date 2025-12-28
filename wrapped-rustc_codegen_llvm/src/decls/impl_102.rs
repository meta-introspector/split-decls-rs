macro_rules! deps {
    () => {
        OwnedTargetMachine!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl Drop for OwnedTargetMachine { fn drop (& mut self) { unsafe { llvm :: LLVMRustDisposeTargetMachine (self . tm_unique . as_ptr ()) ; } } }
    };
}

impl_102!();