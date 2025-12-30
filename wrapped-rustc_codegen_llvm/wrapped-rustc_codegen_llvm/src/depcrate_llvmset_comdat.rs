// Generated macro for set_comdat (function)
macro_rules! Depcrate_llvmset_comdat {
() => {
// Module: crate::llvm
// Provides: {"set_comdat"}
// Dependencies: {}
# [doc = " Get the `name`d comdat from `llmod` and assign it to `llglobal`."] # [doc = ""] # [doc = " Inserts the comdat into `llmod` if it does not exist."] # [doc = " It is an error to call this if the target does not support comdat."] pub (crate) fn set_comdat (llmod : & Module , llglobal : & Value , name : & CStr) { unsafe { let comdat = LLVMGetOrInsertComdat (llmod , name . as_ptr ()) ; LLVMSetComdat (llglobal , comdat) ; } }
};
}
