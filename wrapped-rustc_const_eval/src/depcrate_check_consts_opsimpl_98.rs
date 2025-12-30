// Generated macro for impl_98 (impl)
macro_rules! Depcrate_check_consts_opsimpl_98 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for RawPtrComparison { fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: RawPtrComparisonErr { span }) } }
};
}
