// Generated macro for impl_102 (impl)
macro_rules! Depcrate_check_consts_opsimpl_102 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for ThreadLocalAccess { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: ThreadLocalAccessErr { span }) } }
};
}
