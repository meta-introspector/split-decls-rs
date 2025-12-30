// Generated macro for impl_200 (impl)
macro_rules! Depcrate_back_ltoimpl_200 {
() => {
// Module: crate::back::lto
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'a > Linker < 'a > { pub (crate) fn new (llmod : & 'a llvm :: Module) -> Self { unsafe { Linker (llvm :: LLVMRustLinkerNew (llmod)) } } pub (crate) fn add (& mut self , bytecode : & [u8]) -> Result < () , () > { unsafe { if llvm :: LLVMRustLinkerAdd (self . 0 , bytecode . as_ptr () as * const libc :: c_char , bytecode . len () ,) { Ok (()) } else { Err (()) } } } }
};
}
