// Generated macro for impl_525 (impl)
macro_rules! Depcrate_contextimpl_525 {
() => {
// Module: crate::context
// Provides: {"impl_525"}
// Dependencies: {}
impl < 'll > SimpleCx < 'll > { pub (crate) fn get_type_of_global (& self , val : & 'll Value) -> & 'll Type { unsafe { llvm :: LLVMGlobalGetValueType (val) } } pub (crate) fn val_ty (& self , v : & 'll Value) -> & 'll Type { common :: val_ty (v) } }
};
}
