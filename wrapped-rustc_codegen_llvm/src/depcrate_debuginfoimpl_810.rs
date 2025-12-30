// Generated macro for impl_810 (impl)
macro_rules! Depcrate_debuginfoimpl_810 {
() => {
// Module: crate::debuginfo
// Provides: {"impl_810"}
// Dependencies: {}
impl < 'll > Builder < '_ , 'll , '_ > { pub (crate) fn get_dbg_loc (& self) -> Option < & 'll DILocation > { unsafe { llvm :: LLVMGetCurrentDebugLocation2 (self . llbuilder) } } }
};
}
