// Generated macro for OpITy (type)
macro_rules! Depcrate_opOpITy {
() => {
// Module: crate::op
// Provides: {"OpITy"}
// Dependencies: {}
# [doc = " Access the associated `FTy::Int` type from an op (helper to avoid ambiguous associated types)."] pub type OpITy < Op > = < < Op as MathOp > :: FTy as Float > :: Int ;
};
}
