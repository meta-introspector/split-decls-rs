// Generated macro for impl_92 (impl)
macro_rules! Depcrate_check_consts_opsimpl_92 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for EscapingCellBorrow { fn importance (& self) -> DiagImportance { DiagImportance :: Secondary } fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , span : Span) -> Diag < 'tcx > { ccx . dcx () . create_err (errors :: InteriorMutableBorrowEscaping { span , kind : ccx . const_kind () }) } }
};
}
