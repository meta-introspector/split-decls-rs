// Generated macro for OperandBundleBox (struct)
macro_rules! Depcrate_llvmOperandBundleBox {
() => {
// Module: crate::llvm
// Provides: {"OperandBundleBox"}
// Dependencies: {}
# [doc = " Owning pointer to an [`OperandBundle`] that will dispose of the bundle"] # [doc = " when dropped."] pub (crate) struct OperandBundleBox < 'a > { raw : ptr :: NonNull < OperandBundle < 'a > > , }
};
}
