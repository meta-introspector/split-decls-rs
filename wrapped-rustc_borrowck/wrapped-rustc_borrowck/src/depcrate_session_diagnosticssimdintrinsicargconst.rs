// Generated macro for SimdIntrinsicArgConst (struct)
macro_rules! Depcrate_session_diagnosticsSimdIntrinsicArgConst {
() => {
// Module: crate::session_diagnostics
// Provides: {"SimdIntrinsicArgConst"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (borrowck_simd_intrinsic_arg_const)] pub (crate) struct SimdIntrinsicArgConst { # [primary_span] pub span : Span , pub arg : usize , pub intrinsic : String , }
};
}
