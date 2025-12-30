// Generated macro for impl_219 (impl)
macro_rules! Depcrate_back_ltoimpl_219 {
() => {
// Module: crate::back::lto
// Provides: {"impl_219"}
// Dependencies: {}
impl ThinBufferMethods for ThinBuffer { fn data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustThinLTOBufferPtr (self . 0) as * const _ ; let len = llvm :: LLVMRustThinLTOBufferLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }
};
}
