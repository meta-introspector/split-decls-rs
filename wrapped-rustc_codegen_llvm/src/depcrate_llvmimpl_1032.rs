// Generated macro for impl_1032 (impl)
macro_rules! Depcrate_llvmimpl_1032 {
() => {
// Module: crate::llvm
// Provides: {"impl_1032"}
// Dependencies: {}
impl LLVMRustResult { pub (crate) fn into_result (self) -> Result < () , () > { match self { LLVMRustResult :: Success => Ok (()) , LLVMRustResult :: Failure => Err (()) , } } }
};
}
