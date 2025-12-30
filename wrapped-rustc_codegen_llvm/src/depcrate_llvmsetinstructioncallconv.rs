// Generated macro for SetInstructionCallConv (function)
macro_rules! Depcrate_llvmSetInstructionCallConv {
() => {
// Module: crate::llvm
// Provides: {"SetInstructionCallConv"}
// Dependencies: {}
pub (crate) fn SetInstructionCallConv (instr : & Value , cc : CallConv) { unsafe { LLVMSetInstructionCallConv (instr , cc as c_uint) ; } }
};
}
