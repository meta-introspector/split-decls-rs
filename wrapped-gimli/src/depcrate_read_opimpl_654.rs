// Generated macro for impl_654 (impl)
macro_rules! Depcrate_read_opimpl_654 {
() => {
// Module: crate::read::op
// Provides: {"impl_654"}
// Dependencies: {}
# [cfg (feature = "read")] impl < R : Reader > EvaluationStorage < R > for StoreOnHeap { type Stack = Vec < Value > ; type ExpressionStack = Vec < (R , R) > ; type Result = Vec < Piece < R > > ; }
};
}
