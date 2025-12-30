// Generated macro for val_ty (function)
macro_rules! Depcrate_commonval_ty {
() => {
// Module: crate::common
// Provides: {"val_ty"}
// Dependencies: {}
# [doc = " Get the [LLVM type][Type] of a [`Value`]."] pub (crate) fn val_ty (v : & Value) -> & Type { unsafe { llvm :: LLVMTypeOf (v) } }
};
}
