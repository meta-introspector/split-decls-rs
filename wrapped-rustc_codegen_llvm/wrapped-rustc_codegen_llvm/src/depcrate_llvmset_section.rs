// Generated macro for set_section (function)
macro_rules! Depcrate_llvmset_section {
() => {
// Module: crate::llvm
// Provides: {"set_section"}
// Dependencies: {}
pub (crate) fn set_section (llglobal : & Value , section_name : & CStr) { unsafe { LLVMSetSection (llglobal , section_name . as_ptr ()) ; } }
};
}
