macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a > Linker < 'a > { pub (crate) fn new (llmod : & 'a llvm :: Module) -> Self { unsafe { Linker (llvm :: LLVMRustLinkerNew (llmod)) } } pub (crate) fn add (& mut self , bytecode : & [u8]) -> Result < () , () > { unsafe { if llvm :: LLVMRustLinkerAdd (self . 0 , bytecode . as_ptr () as * const libc :: c_char , bytecode . len () ,) { Ok (()) } else { Err (()) } } } }
    };
}

impl_73!()