// Generated macro for report_simd_type_validation_error (function)
macro_rules! Depcrate_intrinsics_simdreport_simd_type_validation_error {
() => {
// Module: crate::intrinsics::simd
// Provides: {"report_simd_type_validation_error"}
// Dependencies: {}
fn report_simd_type_validation_error (fx : & mut FunctionCx < '_ , '_ , '_ > , intrinsic : Symbol , span : Span , ty : Ty < '_ > ,) { fx . tcx . dcx () . span_err (span , format ! ("invalid monomorphization of `{}` intrinsic: expected SIMD input type, found non-SIMD `{}`" , intrinsic , ty)) ; fx . bcx . ins () . trap (TrapCode :: user (1) . unwrap ()) ; }
};
}
