// Generated macro for impl_80 (impl)
macro_rules! Depcrate_check_consts_opsimpl_80 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for IntrinsicNonConst { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: NonConstIntrinsic { span , name : self . name , kind : ccx . const_kind () , }) } }
};
}
