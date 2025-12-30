// Generated macro for append_module_inline_asm (function)
macro_rules! Depcrate_llvmappend_module_inline_asm {
() => {
// Module: crate::llvm
// Provides: {"append_module_inline_asm"}
// Dependencies: {}
# [doc = " Safe wrapper for `LLVMAppendModuleInlineAsm`, which delegates to"] # [doc = " `Module::appendModuleInlineAsm`."] pub (crate) fn append_module_inline_asm < 'll > (llmod : & 'll Module , asm : & [u8]) { unsafe { LLVMAppendModuleInlineAsm (llmod , asm . as_ptr () , asm . len ()) ; } }
};
}
