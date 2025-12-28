macro_rules! deps {
    () => {
        MemoryEffects!();
    };
}

macro_rules! impl_516 {
    () => {
        deps!();
        impl MemoryEffects { # [doc = " Create an LLVM Attribute with these memory effects."] pub (crate) fn create_attr (self , llcx : & Context) -> & Attribute { unsafe { LLVMRustCreateMemoryEffectsAttr (llcx , self) } } }
    };
}

impl_516!();