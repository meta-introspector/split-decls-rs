// Generated macro for impl_209 (impl)
macro_rules! Depcrate_back_ltoimpl_209 {
() => {
// Module: crate::back::lto
// Provides: {"impl_209"}
// Dependencies: {}
impl ModuleBufferMethods for ModuleBuffer { fn data (& self) -> & [u8] { unsafe { let ptr = llvm :: LLVMRustModuleBufferPtr (self . 0) ; let len = llvm :: LLVMRustModuleBufferLen (self . 0) ; slice :: from_raw_parts (ptr , len) } } }
};
}
