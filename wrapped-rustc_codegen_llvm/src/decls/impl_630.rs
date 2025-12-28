macro_rules! deps {
    () => {
        ModuleLlvm!();
    };
}

macro_rules! impl_630 {
    () => {
        deps!();
        impl Drop for ModuleLlvm { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . tm) ; llvm :: LLVMContextDispose (& mut * (self . llcx as * mut _)) ; } } }
    };
}

impl_630!()