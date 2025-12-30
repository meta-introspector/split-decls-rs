// Generated macro for impl_88 (impl)
macro_rules! Depcrate_check_consts_opsimpl_88 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for InlineAsm { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: UnallowedInlineAsm { span , kind : ccx . const_kind () }) } }
};
}
