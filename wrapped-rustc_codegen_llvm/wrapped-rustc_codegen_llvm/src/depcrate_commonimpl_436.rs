// Generated macro for impl_436 (impl)
macro_rules! Depcrate_commonimpl_436 {
() => {
// Module: crate::common
// Provides: {"impl_436"}
// Dependencies: {}
impl < 'll > Funclet < 'll > { pub (crate) fn new (cleanuppad : & 'll Value) -> Self { Funclet { cleanuppad , operand : llvm :: OperandBundleBox :: new ("funclet" , & [cleanuppad]) } } pub (crate) fn cleanuppad (& self) -> & 'll Value { self . cleanuppad } pub (crate) fn bundle (& self) -> & llvm :: OperandBundle < 'll > { self . operand . as_ref () } }
};
}
