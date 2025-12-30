// Generated macro for impl_1037 (impl)
macro_rules! Depcrate_mirimpl_1037 {
() => {
// Module: crate::mir
// Provides: {"impl_1037"}
// Dependencies: {}
impl < 'tcx , V : CodegenObject > LocalRef < 'tcx , V > { fn new_operand (layout : TyAndLayout < 'tcx >) -> LocalRef < 'tcx , V > { if layout . is_zst () { LocalRef :: Operand (OperandRef :: zero_sized (layout)) } else { LocalRef :: PendingOperand } } }
};
}
