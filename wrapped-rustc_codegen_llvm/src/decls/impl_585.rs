macro_rules! impl_585 {
    () => {
        impl Type { # [doc = " Creates an integer type with the given number of bits, e.g., i24"] pub (crate) fn ix_llcx (llcx : & llvm :: Context , num_bits : u64) -> & Type { unsafe { llvm :: LLVMIntTypeInContext (llcx , num_bits as c_uint) } } pub (crate) fn ptr_llcx (llcx : & llvm :: Context) -> & Type { unsafe { llvm :: LLVMPointerTypeInContext (llcx , AddressSpace :: ZERO . 0) } } }
    };
}

impl_585!();