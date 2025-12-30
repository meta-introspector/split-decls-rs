// Generated macro for try_as_const_integral (function)
macro_rules! Depcrate_commontry_as_const_integral {
() => {
// Module: crate::common
// Provides: {"try_as_const_integral"}
// Dependencies: {}
fn try_as_const_integral (v : & Value) -> Option < & ConstantInt > { unsafe { llvm :: LLVMIsAConstantInt (v) } }
};
}
