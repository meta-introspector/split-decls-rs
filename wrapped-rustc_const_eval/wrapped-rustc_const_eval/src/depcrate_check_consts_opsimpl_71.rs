// Generated macro for impl_71 (impl)
macro_rules! Depcrate_check_consts_opsimpl_71 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for FnCallIndirect { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: UnallowedFnPointerCall { span , kind : ccx . const_kind () }) } }
};
}
