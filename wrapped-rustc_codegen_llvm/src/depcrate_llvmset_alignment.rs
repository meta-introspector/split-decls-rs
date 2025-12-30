// Generated macro for set_alignment (function)
macro_rules! Depcrate_llvmset_alignment {
() => {
// Module: crate::llvm
// Provides: {"set_alignment"}
// Dependencies: {}
pub (crate) fn set_alignment (llglobal : & Value , align : Align) { unsafe { ffi :: LLVMSetAlignment (llglobal , align . bytes () as c_uint) ; } }
};
}
