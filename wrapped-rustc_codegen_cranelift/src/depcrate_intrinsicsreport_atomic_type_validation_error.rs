// Generated macro for report_atomic_type_validation_error (function)
macro_rules! Depcrate_intrinsicsreport_atomic_type_validation_error {
() => {
// Module: crate::intrinsics
// Provides: {"report_atomic_type_validation_error"}
// Dependencies: {}
fn report_atomic_type_validation_error < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , intrinsic : Symbol , span : Span , ty : Ty < 'tcx > ,) { fx . tcx . dcx () . span_err (span , format ! ("`{}` intrinsic: expected basic integer or raw pointer type, found `{:?}`" , intrinsic , ty) ,) ; fx . bcx . ins () . trap (TrapCode :: user (1) . unwrap ()) ; }
};
}
