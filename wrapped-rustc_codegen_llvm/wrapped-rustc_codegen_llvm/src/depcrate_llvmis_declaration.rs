// Generated macro for is_declaration (function)
macro_rules! Depcrate_llvmis_declaration {
() => {
// Module: crate::llvm
// Provides: {"is_declaration"}
// Dependencies: {}
pub (crate) fn is_declaration (llglobal : & Value) -> bool { unsafe { LLVMIsDeclaration (llglobal) } . is_true () }
};
}
