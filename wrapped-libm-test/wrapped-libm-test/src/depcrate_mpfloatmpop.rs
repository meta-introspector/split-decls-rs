// Generated macro for MpOp (trait)
macro_rules! Depcrate_mpfloatMpOp {
() => {
// Module: crate::mpfloat
// Provides: {"MpOp"}
// Dependencies: {}
# [doc = " Structures that represent a float operation."] # [doc = ""] pub trait MpOp : MathOp { # [doc = " The struct itself should hold any context that can be reused among calls to `run` (allocated"] # [doc = " `MpFloat`s)."] type MpTy ; # [doc = " Create a new instance."] fn new_mp () -> Self :: MpTy ; # [doc = " Perform the operation."] # [doc = ""] # [doc = " Usually this means assigning inputs to cached floats, performing the operation, applying"] # [doc = " subnormal approximation, and converting the result back to concrete values."] fn run (this : & mut Self :: MpTy , input : Self :: RustArgs) -> Self :: RustRet ; }
};
}
