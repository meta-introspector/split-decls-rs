macro_rules! deps {
    () => {
        OperandBundleBox!();
    };
}

macro_rules! impl_539 {
    () => {
        deps!();
        impl Drop for OperandBundleBox < '_ > { fn drop (& mut self) { unsafe { LLVMDisposeOperandBundle (self . raw) ; } } }
    };
}

impl_539!();