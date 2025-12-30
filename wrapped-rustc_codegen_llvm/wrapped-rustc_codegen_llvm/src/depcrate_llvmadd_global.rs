// Generated macro for add_global (function)
macro_rules! Depcrate_llvmadd_global {
() => {
// Module: crate::llvm
// Provides: {"add_global"}
// Dependencies: {}
pub (crate) fn add_global < 'a > (llmod : & 'a Module , ty : & 'a Type , name_cstr : & CStr) -> & 'a Value { unsafe { LLVMAddGlobal (llmod , ty , name_cstr . as_ptr ()) } }
};
}
