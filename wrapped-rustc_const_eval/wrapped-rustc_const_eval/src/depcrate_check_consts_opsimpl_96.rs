// Generated macro for impl_96 (impl)
macro_rules! Depcrate_check_consts_opsimpl_96 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for PanicNonStr { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: PanicNonStrErr { span }) } }
};
}
