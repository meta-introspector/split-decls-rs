// Generated macro for to_llvm_code_model (function)
macro_rules! Depcrate_back_writeto_llvm_code_model {
() => {
// Module: crate::back::write
// Provides: {"to_llvm_code_model"}
// Dependencies: {}
pub (crate) fn to_llvm_code_model (code_model : Option < CodeModel >) -> llvm :: CodeModel { match code_model { Some (CodeModel :: Tiny) => llvm :: CodeModel :: Tiny , Some (CodeModel :: Small) => llvm :: CodeModel :: Small , Some (CodeModel :: Kernel) => llvm :: CodeModel :: Kernel , Some (CodeModel :: Medium) => llvm :: CodeModel :: Medium , Some (CodeModel :: Large) => llvm :: CodeModel :: Large , None => llvm :: CodeModel :: None , } }
};
}
