// Generated macro for AtomicRmwOp (enum)
macro_rules! Depcrate_intrinsics_atomicAtomicRmwOp {
() => {
// Module: crate::intrinsics::atomic
// Provides: {"AtomicRmwOp"}
// Dependencies: {}
pub enum AtomicRmwOp { MirOp { op : mir :: BinOp , # [doc = " Indicates whether the result of the operation should be negated (`UnOp::Not`, must be a"] # [doc = " boolean-typed operation)."] neg : bool , } , Max , Min , }
};
}
