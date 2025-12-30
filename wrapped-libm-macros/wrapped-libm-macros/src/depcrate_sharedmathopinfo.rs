// Generated macro for MathOpInfo (struct)
macro_rules! Depcrate_sharedMathOpInfo {
() => {
// Module: crate::shared
// Provides: {"MathOpInfo"}
// Dependencies: {}
# [doc = " Combined information about a function implementation."] # [derive (Debug , Clone)] pub struct MathOpInfo { pub name : & 'static str , pub float_ty : FloatTy , # [doc = " Function signature for C implementations"] pub c_sig : Signature , # [doc = " Function signature for Rust implementations"] pub rust_sig : Signature , # [doc = " True if part of libm's public API"] pub public : bool , }
};
}
