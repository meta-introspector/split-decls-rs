// Generated macro for AtomicOrdering (enum)
macro_rules! Depcrate_llvm_ffiAtomicOrdering {
() => {
// Module: crate::llvm::ffi
// Provides: {"AtomicOrdering"}
// Dependencies: {}
# [doc = " LLVMAtomicOrdering"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum AtomicOrdering { # [allow (dead_code)] NotAtomic = 0 , # [allow (dead_code)] Unordered = 1 , Monotonic = 2 , Acquire = 4 , Release = 5 , AcquireRelease = 6 , SequentiallyConsistent = 7 , }
};
}
