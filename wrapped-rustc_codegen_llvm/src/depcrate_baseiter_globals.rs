// Generated macro for iter_globals (function)
macro_rules! Depcrate_baseiter_globals {
() => {
// Module: crate::base
// Provides: {"iter_globals"}
// Dependencies: {}
pub (crate) fn iter_globals (llmod : & llvm :: Module) -> ValueIter < '_ > { unsafe { ValueIter { cur : llvm :: LLVMGetFirstGlobal (llmod) , step : llvm :: LLVMGetNextGlobal } } }
};
}
